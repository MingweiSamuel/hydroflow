use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, ready};

use memo_map::MemoMap;
use russh::client::{self, Config, Handle, Handler, Msg};
use russh::keys::{PrivateKeyWithHashAlg, load_secret_key, ssh_key};
use russh::{ChannelMsg, ChannelWriteHalf, CryptoVec, Disconnect};
use tokio::io::{AsyncBufRead, AsyncRead, ReadBuf};
use tokio::net::ToSocketAddrs;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

struct NoCheckHandler;
impl Handler for NoCheckHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        // TODO(mingwei): we should check the server's public key fingerprint here, but ssh `publickey`
        // authentication already generally prevents MITM attacks.
        Ok(true)
    }
}

// https://github.com/Eugeny/russh/blob/main/russh/examples/client_exec_simple.rs
/// This struct is a convenience wrapper
/// around a russh client
pub struct Session {
    session: Handle<NoCheckHandler>,
}

impl Session {
    pub async fn connect(
        key_path: impl AsRef<Path>,
        user: impl Into<String>,
        addrs: impl ToSocketAddrs,
    ) -> Result<Self, russh::Error> {
        let config = Arc::new(Config::default()); // Has sane defaults.

        let key_pair = load_secret_key(key_path, None)?;

        let mut session = client::connect(config, addrs, NoCheckHandler).await?;

        // use publickey authentication.
        let auth_res = session
            .authenticate_publickey(
                user,
                PrivateKeyWithHashAlg::new(
                    Arc::new(key_pair),
                    session.best_supported_rsa_hash().await?.flatten(),
                ),
            )
            .await?;

        if auth_res.success() {
            Ok(Self { session })
        } else {
            Err(russh::Error::NotAuthenticated)
        }
    }

    pub async fn channel_open(&self) -> Result<Channel, russh::Error> {
        let channel = self.session.channel_open_session().await?;
        Ok(Channel::from_inner(channel))
    }

    // async fn call(&mut self, command: &str) -> Result<u32> {
    //     let mut channel = self.session.channel_open_session().await?;
    //     channel.exec(true, command).await?;

    //     let mut code = None;
    //     let mut stdout = tokio::io::stdout();

    //     loop {
    //         // There's an event available on the session channel
    //         let Some(msg) = channel.wait().await else {
    //             break;
    //         };
    //         match msg {
    //             // Write data to the terminal
    //             ChannelMsg::Data { ref data } => {
    //                 // TODO!!!
    //                 // TODO: also handle DataExt
    //                 // stdout.write_all(data).await?;
    //                 // stdout.flush().await?;
    //             }
    //             // The command has returned an exit code
    //             ChannelMsg::ExitStatus { exit_status } => {
    //                 code = Some(exit_status);
    //                 // cannot leave the loop immediately, there might still be more data to receive
    //             }
    //             _ => {}
    //         }
    //     }
    //     Ok(code.expect("program did not exit cleanly"))
    // }

    pub async fn close(&mut self) -> Result<(), russh::Error> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

pub struct Channel {
    write_half: ChannelWriteHalf<Msg>,
    data_receivers: Arc<MemoMap<Option<u32>, mpsc::UnboundedSender<CryptoVec>>>,
    recv_exit: oneshot::Receiver<u32>,
    reader: JoinHandle<()>,
}
impl Channel {
    fn from_inner(inner: russh::Channel<Msg>) -> Self {
        let (mut read_half, write_half) = inner.split();
        let (send_exit, recv_exit) = oneshot::channel();
        let data_receivers = Arc::new(MemoMap::<_, mpsc::UnboundedSender<CryptoVec>>::new());
        let send_data_receivers = data_receivers.clone();

        let reader = tokio::task::spawn(async move {
            let mut send_exit = Some(send_exit);
            while let Some(msg) = read_half.wait().await {
                let (data, ext) = match msg {
                    // Write data to the terminal
                    ChannelMsg::Data { data } => (data, None),
                    ChannelMsg::ExtendedData { data, ext } => (data, Some(ext)),
                    // The command has returned an exit code
                    ChannelMsg::ExitStatus { exit_status } => {
                        if let Some(send_exit) = send_exit.take() {
                            let _ = send_exit.send(exit_status);
                        }
                        // cannot leave the loop immediately, there might still be more data to receive
                        continue;
                    }
                    ChannelMsg::Eof => continue, // TODO(mingwei)
                    _ => continue,
                };

                if let Some(send) = send_data_receivers.get(&ext) {
                    let _ = send.send(data);
                }
            }
        });

        Self {
            write_half,
            data_receivers,
            recv_exit,
            reader,
        }
    }

    pub fn read_stream(&self, ext: Option<u32>) -> Option<ReadStream> {
        let (send, recv) = mpsc::unbounded_channel();
        let added = self.data_receivers.insert(ext, send);
        if !added {
            return None;
        }
        Some(ReadStream { recv, curr: None })
    }

    pub fn write_stream(&self, ext: Option<u32>) -> Option<impl AsyncWrite> {
        // self.write_half.make_
    }

    pub async fn exec(&self, command: impl Into<Vec<u8>>) -> Result<(), russh::Error> {
        self.write_half.exec(false, command).await
    }

    pub fn read_stdout(&self) -> ReadStream {
        self.read_stream(None).unwrap()
    }

    pub fn read_stderr(&self) -> ReadStream {
        self.read_stream(Some(1)).unwrap()
    }

    pub async fn close(&self) -> Result<(), russh::Error> {
        self.write_half.close().await
    }

    // // TODO(mingwei): ownership issue
    // pub async fn exit_status(&self) -> Result<u32, russh::Error> {
    //     self.recv_exit.await.map_err(|_| russh::Error::Disconnect)
    // }
}

pub struct ReadStream {
    recv: mpsc::UnboundedReceiver<CryptoVec>,
    curr: Option<(CryptoVec, usize)>,
}
impl AsyncRead for ReadStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // Defer to the underlying `AsyncBufRead` implementation.
        let read_buf = ready!(self.as_mut().poll_fill_buf(cx))?;
        let amt = std::cmp::min(read_buf.len(), buf.capacity());
        buf.put_slice(&read_buf[..amt]);
        self.consume(amt);
        Poll::Ready(Ok(()))
    }
}
impl AsyncBufRead for ReadStream {
    fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<&[u8]>> {
        let this = self.get_mut();

        if this.curr.is_none() {
            match this.recv.poll_recv(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(opt_data) => {
                    this.curr = opt_data.map(|data| (data, 0));
                }
            }
        }

        Poll::Ready(Ok(this
            .curr
            .as_ref()
            .map(|(buf, offset)| &buf[*offset..])
            .unwrap_or(&[])))
    }

    fn consume(self: Pin<&mut Self>, amt: usize) {
        let this = self.get_mut();
        if let Some((buf, offset)) = &mut this.curr {
            *offset += amt;
            debug_assert!(*offset <= buf.len());
            if *offset == buf.len() {
                this.curr = None;
            }
        } else {
            debug_assert!(amt == 0);
        }
    }
}
