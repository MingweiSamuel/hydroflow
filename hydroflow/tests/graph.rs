#[test]
fn test_doctest() {
    let output = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let output_inner = output.clone();

    use hydroflow::scheduled::graph::Hydroflow;
    use hydroflow::scheduled::graph_ext::GraphExt;
    use hydroflow::scheduled::handoff::VecHandoff;
    use hydroflow::tl;

    let mut hf = Hydroflow::new();

    let (loop_send_port, loop_recv_port) = hf.make_edge::<_, VecHandoff<usize>>("loop handoff");

    let (input_send_port, input_recv_port) =
        hf.make_edge::<_, VecHandoff<usize>>("external input handoff");
    let external_input = hf.add_input::<_, _, VecHandoff<usize>>("external input", input_send_port);

    hf.add_subgraph(
        "incrementer",
        tl!(input_recv_port, loop_recv_port),
        tl!(loop_send_port),
        move |_context, tl!(input_recv_ctx, loop_recv_ctx), tl!(loop_send_ctx)| {
            let input_buffer = input_recv_ctx.take_inner();
            let loop_buffer = loop_recv_ctx.take_inner();
            input_buffer
                .into_iter()
                .chain(loop_buffer)
                .inspect(|&x| output_inner.borrow_mut().push(x))
                .filter(|&x| x < 10)
                .for_each(|x| {
                    loop_send_ctx.give(Some(x + 1));
                });
        },
    );

    external_input.give(Some(1));
    external_input.flush(); // Make sure to flush!

    hf.tick(); // Runs the Hydroflow until no more work can be done.

    assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &**output.borrow());
}
