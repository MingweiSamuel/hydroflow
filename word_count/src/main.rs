use std::io::BufRead;

use hydroflow::compiled::pull::JoinState;
use hydroflow::compiled::pull::SymmetricHashJoin;
use hydroflow::compiled::{ForEach, Pivot, Tee};
use hydroflow::lang::collections::Iter;
use hydroflow::scheduled::handoff::VecHandoff;
use hydroflow::scheduled::Hydroflow;
use hydroflow::{tl, tlt};

fn main() {
    let mut hf = Hydroflow::new();

    let lines_out = hf.add_source::<_, VecHandoff<String>>(|ctx, send| {
        send.give(Iter(std::io::stdin().lock().lines().map(|line| line.expect("Failed to parse UTF-8."))));
    });

    let (lines_in, words_pairs_out) = hf.add_inout::<_, VecHandoff<String>, VecHandoff<(String, usize)>>(|ctx, recv, send| {
        for line in recv.take_inner() {
            send.give(Iter(line.split(' ').map(|word| word.to_owned()).map(|word| (word, 1))));
        }
    });

    hf.add_edge(lines_out, lines_in);
}
