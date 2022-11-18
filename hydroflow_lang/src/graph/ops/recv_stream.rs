use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_0, RANGE_1,
};

use proc_macro2::Span;
use quote::quote_spanned;

#[hydroflow_internalmacro::operator_docgen]
pub const RECV_STREAM: OperatorConstraints = OperatorConstraints {
    name: "recv_stream",
    hard_range_inn: RANGE_0,
    soft_range_inn: RANGE_0,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: None,
    ports_out: None,
    num_args: 1,
    input_delaytype_fn: &|_| None,
    write_fn: &(|wc @ &WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident, arguments, ..
                 }| {
        let receiver = &arguments[0];
        let stream_ident = wc.make_ident("stream");
        let write_prologue = quote_spanned! {op_span=>
            let mut #stream_ident = Box::pin(#receiver);
        };
        let write_iterator = quote_spanned! {op_span=>
            let #ident = std::iter::from_fn(|| {
                match #root::futures::stream::Stream::poll_next(#stream_ident.as_mut(), &mut std::task::Context::from_waker(&context.waker())) {
                    std::task::Poll::Ready(maybe) => maybe,
                    std::task::Poll::Pending => None,
                }
            });
        };
        OperatorWriteOutput {
            write_prologue,
            write_iterator,
            ..Default::default()
        }
    }),
};
