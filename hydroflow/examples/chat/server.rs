use crate::{GraphType, Opts};

use crate::helpers::{deserialize_msg, serialize_msg};
use crate::protocol::Message;

use hydroflow::hydroflow_syntax;
use hydroflow::scheduled::graph::Hydroflow;
use tokio::net::UdpSocket;

pub(crate) async fn run_server(opts: Opts) {
    // First, set up the socket
    let server_socket = UdpSocket::bind(("127.0.0.1", opts.port)).await.unwrap();
    let (outbound, inbound) = hydroflow::util::udp_lines(server_socket);
    println!("Server live!");

    let mut df: Hydroflow = hydroflow_syntax! {
        // NW channels
        outbound_chan = merge()
            -> map(|(msg, addr)| (serialize_msg(msg), addr))
            -> sink_async(outbound);

        inbound_chan = recv_stream(inbound) -> map(deserialize_msg::<Message>) -> split(); // tee();
        // ConnectRequest
        members = inbound_chan[0] -> flatten() -> map(|(_nickname, addr)| addr) -> tee();
        // ChatMsg
        msgs = inbound_chan[1] -> flatten()
            -> map(|(nickname, message, ts)| Message::ChatMsg { nickname, message, ts });
        // ConnectResponse
        inbound_chan[2] -> null();

        // Logic
        members[0] -> map(|addr| (Message::ConnectResponse, addr)) -> [0]outbound_chan;
        broadcast = cross_join() -> [1]outbound_chan;
        msgs -> [0]broadcast;
        members[1] -> [1]broadcast;
    };

    if let Some(graph) = opts.graph {
        match graph {
            GraphType::Mermaid => {
                println!("{}", df.generate_mermaid())
            }
            GraphType::Dot => {
                println!("{}", df.generate_dot())
            }
            GraphType::Json => {
                println!("{}", df.generate_json())
            }
        }
    }

    df.run_async().await.unwrap();
}
