use babyflow::babyflow::Query;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hydroflow::scheduled::collections::Iter;
use hydroflow::scheduled::ctx::{RecvCtx, SendCtx};
use hydroflow::scheduled::handoff::VecHandoff;
use hydroflow::scheduled::query::Query as Q;
use hydroflow::scheduled::Hydroflow;
use timely::dataflow::operators::{Concatenate, Filter, Inspect, ToStream};

const NUM_OPS: usize = 20;
const NUM_INTS: usize = 100_000;
const BRANCH_FACTOR: usize = 2;

fn benchmark_hydroflow(c: &mut Criterion) {
    c.bench_function("fork_join/hydroflow", |b| {
        b.iter(|| {
            let mut df = Hydroflow::new();

            let mut sent = false;
            let source = df.add_source(move |send: &SendCtx<VecHandoff<_>>| {
                if !sent {
                    sent = true;
                    send.give(Iter(0..NUM_INTS));
                }
            });

            let (tee_in, mut out1, mut out2) = df.add_binary_out(
                |recv: &RecvCtx<VecHandoff<_>>,
                 send1: &SendCtx<VecHandoff<_>>,
                 send2: &SendCtx<VecHandoff<_>>| {
                    for v in recv.take_inner().into_iter() {
                        if v % 2 == 0 {
                            send1.give(Some(v));
                        } else {
                            send2.give(Some(v));
                        }
                    }
                },
            );

            df.add_edge(source, tee_in);
            for _ in 0..NUM_OPS {
                let (in1, in2, mut new_out1, mut new_out2) = df.add_binary_in_binary_out(
                    move |recv1: &RecvCtx<VecHandoff<_>>,
                          recv2: &RecvCtx<VecHandoff<_>>,
                          send1,
                          send2| {
                        for v in recv1
                            .take_inner()
                            .into_iter()
                            .chain(recv2.take_inner().into_iter())
                        {
                            if v % 2 == 0 {
                                send1.give(Some(v));
                            } else {
                                send2.give(Some(v));
                            }
                        }
                    },
                );
                std::mem::swap(&mut out1, &mut new_out1);
                std::mem::swap(&mut out2, &mut new_out2);
                df.add_edge(new_out1, in1);
                df.add_edge(new_out2, in2);
            }

            let (sink1, sink2) = df.add_binary_sink(
                |recv1: &RecvCtx<VecHandoff<_>>, recv2: &RecvCtx<VecHandoff<_>>| {
                    for x in recv1.take_inner() {
                        black_box(x);
                    }
                    for x in recv2.take_inner() {
                        black_box(x);
                    }
                },
            );

            df.add_edge(out1, sink1);
            df.add_edge(out2, sink2);

            df.tick()
        })
    });
}

fn benchmark_hydroflow_builder(c: &mut Criterion) {
    c.bench_function("fork_join/hydroflow_builder", |b| {
        b.iter(|| {
            // TODO(justin): this creates more operators than necessary.
            let mut q = Q::new();

            let mut source = q.source(|send| {
                send.give(Iter(0..NUM_INTS));
            });

            for _ in 0..NUM_OPS {
                let mut outs = source.tee(2).into_iter();
                let (mut out1, mut out2) = (outs.next().unwrap(), outs.next().unwrap());
                out1 = out1.filter(|x| x % 2 == 0);
                out2 = out2.filter(|x| x % 2 == 1);
                source = out1.concat(out2);
            }

            source.sink(|v| {
                black_box(v);
            });

            q.tick();
        })
    });
}

fn benchmark_raw(c: &mut Criterion) {
    c.bench_function("fork_join/raw", |b| {
        b.iter(|| {
            let mut evn = Vec::new();
            let mut odd = Vec::new();

            let mut data: Vec<_> = (0..NUM_INTS).collect();

            for _ in 0..NUM_OPS {
                for i in data.drain(..) {
                    if i % 2 == 0 {
                        evn.push(i);
                    } else {
                        odd.push(i)
                    }
                }

                data.append(&mut evn);
                data.append(&mut odd);
            }
        })
    });
}

fn benchmark_babyflow(c: &mut Criterion) {
    c.bench_function("fork_join/babyflow", |b| {
        b.iter(|| {
            let mut q = Query::new();

            let mut op = q.source(move |send| {
                send.give_iterator(0..NUM_INTS);
            });

            for _ in 0..NUM_OPS {
                op = q.concat(
                    (0..BRANCH_FACTOR).map(|i| op.clone().filter(move |x| x % BRANCH_FACTOR == i)),
                );
            }

            op.sink(|i| {
                black_box(i);
            });

            (*q.df).borrow_mut().run();
        })
    });
}

fn benchmark_timely(c: &mut Criterion) {
    c.bench_function("fork_join/timely", |b| {
        b.iter(|| {
            timely::example(|scope| {
                let mut op = (0..NUM_INTS).to_stream(scope);
                for _ in 0..NUM_OPS {
                    let mut ops = Vec::new();

                    for i in 0..BRANCH_FACTOR {
                        ops.push(op.filter(move |x| x % BRANCH_FACTOR == i))
                    }

                    op = scope.concatenate(ops);
                }

                op.inspect(|i| {
                    black_box(i);
                });
            });
        })
    });
}

fn benchmark_spinachflow_asym(c: &mut Criterion) {
    c.bench_function("fork_join/spinachflow (asymmetric)", |b| {
        b.to_async(
            tokio::runtime::Builder::new_current_thread()
                .build()
                .unwrap(),
        )
        .iter(|| {
            async {
                use spinachflow::futures::StreamExt;
                use spinachflow::futures::future::ready;

                let stream = spinachflow::futures::stream::iter(0..NUM_INTS);

                ///// MAGIC NUMBER!!!!!!!! is NUM_OPS
                seq_macro::seq!(N in 0..20 {
                    let mut asym_split = spinachflow::stream::AsymSplit::new(stream);
                    let mut i = 0;
                    let splits = [(); BRANCH_FACTOR - 1].map(|_| {
                        i += 1;
                        asym_split.add_split().filter(move |x| ready(i == x % BRANCH_FACTOR))
                    });
                    let stream = spinachflow::stream::SelectArr::new(splits);

                    let asym_split = asym_split.filter(|x| ready(0 == x % BRANCH_FACTOR));
                    let stream = spinachflow::futures::stream::select(asym_split, stream);
                    let stream: std::pin::Pin<Box<dyn spinachflow::futures::Stream<Item = usize>>> = Box::pin(stream);
                });

                let mut stream = stream;
                loop {
                    let item = stream.next().await;
                    if item.is_none() {
                        break;
                    }
                }
            }
        });
    });
}

fn benchmark_pyro_plumbing(c: &mut Criterion) {
    use std::cell::RefCell;

    use pyro::{Context, Pyro};

    type Handoff = RefCell<Vec<usize>>;

    c.bench_function("fork_join/pyro_plumbing", |b| {
        b.iter(|| {
            let mut pyro = Pyro::new();

            let mut handoff_evn = pyro.default_state::<Handoff>();
            let mut handoff_odd = pyro.default_state::<Handoff>();

            let mut next_tid = pyro.new_task(move |mut ctx: Context<'_>| {
                let input_evn = std::mem::take(ctx.get_state_mut(handoff_evn).get_mut()).into_iter();
                let input_odd = std::mem::take(ctx.get_state_mut(handoff_odd).get_mut()).into_iter();
                for x in input_evn.chain(input_odd) {
                    black_box(x);
                }
            });

            for _ in 0..NUM_OPS {
                let pred_handoff_evn = pyro.default_state::<Handoff>();
                let pred_handoff_odd = pyro.default_state::<Handoff>();

                let fj_tid = pyro.new_task(move |mut ctx: Context<'_>| {
                    let should_schedule = {
                        let input_evn = std::mem::take(ctx.get_state_mut(pred_handoff_evn).get_mut()).into_iter();
                        let input_odd = std::mem::take(ctx.get_state_mut(pred_handoff_odd).get_mut()).into_iter();
                        let mut outpt_evn = ctx.get_state_ref(handoff_evn).borrow_mut();
                        let mut outpt_odd = ctx.get_state_ref(handoff_odd).borrow_mut();
                        for x in input_evn.chain(input_odd) {
                            if x % 2 == 0 {
                                outpt_evn.push(x);
                            } else {
                                outpt_odd.push(x);
                            }
                        }
                        !outpt_evn.is_empty() || !outpt_odd.is_empty()
                    };
                    if should_schedule {
                        ctx.schedule(next_tid);
                    }
                });

                handoff_evn = pred_handoff_evn;
                handoff_odd = pred_handoff_odd;
                next_tid = fj_tid;
            }

            let source_handoff = pyro.default_state::<Handoff>();
            let split_tid = pyro.new_task(move |mut ctx: Context<'_>| {
                let should_schedule = {
                    let mut input = ctx.get_state_ref(source_handoff).borrow_mut();
                    let mut outpt_evn = ctx.get_state_ref(handoff_evn).borrow_mut();
                    let mut outpt_odd = ctx.get_state_ref(handoff_odd).borrow_mut();
                    for x in input.drain(..) {
                        if x % 2 == 0 {
                            outpt_evn.push(x);
                        } else {
                            outpt_odd.push(x);
                        }
                    }
                    !outpt_evn.is_empty() || !outpt_odd.is_empty()
                };
                if should_schedule {
                    ctx.schedule(next_tid);
                }
            });

            let source_tid = pyro.new_task(move |mut ctx: Context<'_>| {
                ctx.get_state_mut(source_handoff)
                    .get_mut()
                    .extend(0..NUM_INTS);
                ctx.schedule(split_tid);
            });

            pyro.schedule(source_tid);
            pyro.tick();
        });
    });
}

criterion_group!(
    fork_join_dataflow,
    benchmark_hydroflow,
    benchmark_hydroflow_builder,
    benchmark_babyflow,
    benchmark_timely,
    benchmark_raw,
    benchmark_spinachflow_asym,
    benchmark_pyro_plumbing,
);
criterion_main!(fork_join_dataflow);
