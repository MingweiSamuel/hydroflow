use std::time::Duration;

use hydroflow::scheduled::graph::Hydroflow;

#[test]
pub fn test_echo() {
    let (lines_send, lines_recv) = hydroflow::util::unbounded_channel::<String>();
    let mut stdout_lines = tokio::io::stdout();
    let mut df: Hydroflow = {
        use hydroflow::tl;
        let mut df = hydroflow :: scheduled :: graph :: Hydroflow :: new_with_graph ("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":\"op_1v1: recv_stream(lines_recv)\",\"version\":1},{\"value\":\"op_2v1: map(| line | line + \\\"\\\\n\\\")\",\"version\":1},{\"value\":\"op_3v1: send_async(stdout_lines)\",\"version\":1},{\"value\":\"hoff_4v1_send: handoff\",\"version\":1}],\"edges\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":null,\"version\":0},{\"value\":[{\"idx\":3,\"version\":1}],\"version\":1}],\"handoffs\":[{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":true,\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1}]}\n") ;
        let (hoff_4v1_send, hoff_4v1_recv) = df
            .make_edge::<_, hydroflow::scheduled::handoff::VecHandoff<_>>(
                "handoff GraphNodeId(4v1)",
            );
        let mut sg_1v1_node_1v1_stream = Box::pin(lines_recv);
        df.add_subgraph_stratified(
            "Subgraph GraphSubgraphId(1v1)",
            0,
            (),
            (hoff_4v1_send, ()),
            move |context, (), (hoff_4v1_send, ())| {
                let hoff_4v1_send = hydroflow::pusherator::for_each::ForEach::new(|v| {
                    hoff_4v1_send.give(Some(v));
                });
                let op_1v1 = std::iter::from_fn(|| {
                    match hydroflow::futures::stream::Stream::poll_next(
                        sg_1v1_node_1v1_stream.as_mut(),
                        &mut std::task::Context::from_waker(&context.waker()),
                    ) {
                        std::task::Poll::Ready(maybe) => maybe,
                        std::task::Poll::Pending => None,
                    }
                });
                let op_2v1 = op_1v1.map(|line| line + "\n");
                hydroflow::pusherator::pivot::Pivot::new(op_2v1, hoff_4v1_send).run();
            },
        );
        let mut sg_2v1_node_3v1_buffer: std::collections::VecDeque<_> = Default::default();
        // Option<hydroflow::tokio::io::util::write_all::WriteAll<_, _>>
        let mut sg_2v1_node_3v1_future: Option<std::pin::Pin<std::boxed::Box<_>>> = None;
        df.add_subgraph_stratified(
            "Subgraph GraphSubgraphId(2v1)",
            0,
            (hoff_4v1_recv, ()),
            (),
            move |context, (hoff_4v1_recv, ()), ()| {
                let hoff_4v1_recv = hoff_4v1_recv.take_inner().into_iter();
                let op_3v1 = hydroflow::pusherator::for_each::ForEach::new(|item| {
                    sg_2v1_node_3v1_buffer.push_back(item);
                    while let Some(fut) = &mut sg_2v1_node_3v1_future {
                        match std::future::Future::poll(
                            std::pin::Pin::as_mut(fut),
                            &mut std::task::Context::from_waker(&context.waker()),
                        ) {
                            std::task::Poll::Ready(Ok(())) => {
                                sg_2v1_node_3v1_future = sg_2v1_node_3v1_buffer
                                    .pop_front()
                                    .map(|next_item| {
                                        hydroflow::tokio::io::AsyncWriteExt::write_all(
                                            &mut stdout_lines,
                                            "123abc".as_bytes(),
                                        )
                                    })
                                    .map(std::boxed::Box::pin);
                            }
                            std::task::Poll::Ready(Err(io_err)) => {
                                panic!("send_async IO error {:?}", io_err);
                            }
                            std::task::Poll::Pending => {
                                break;
                            }
                        }
                    }
                });
                hydroflow::pusherator::pivot::Pivot::new(hoff_4v1_recv, op_3v1).run();
            },
        );
        df
    };
    // {
    //     ::std::io::_print(::core::fmt::Arguments::new_v1(
    //         &["", "\n"],
    //         &[::core::fmt::ArgumentV1::new_display(
    //             &df.serde_graph()
    //                 .expect("No graph found, maybe failed to parse.")
    //                 .to_mermaid(),
    //         )],
    //     ));
    // };
    df.run_available();
    lines_send.send("Hello".to_owned()).unwrap();
    lines_send.send("World".to_owned()).unwrap();
    df.run_available();
    lines_send.send("Hello".to_owned()).unwrap();
    lines_send.send("World".to_owned()).unwrap();
    df.run_available();
    std::thread::sleep(Duration::from_secs(1));
}
