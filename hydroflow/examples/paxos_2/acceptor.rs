use std::collections::{HashMap, HashSet};

use super::*;

use hydroflow::scheduled::graph::Hydroflow;

type Output = (
    Hydroflow,
    mpsc::UnboundedSender<Option<P1A>>,
    mpsc::UnboundedReceiver<Option<P1B>>,
    mpsc::UnboundedReceiver<Option<P1BLog>>,
    mpsc::UnboundedSender<Option<P2A>>,
    mpsc::UnboundedReceiver<Option<P2B>>,
);

// # acceptor
// ######################## relation definitions
// # EDB

// id(id)
pub fn acceptor(/* EDB: id(id) */ id: AcceptorId) -> Output {
    let mut builder = HydroflowBuilder::new();

    // p1a(proposerID, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
    let (p1a_send, p1a_recv) = mpsc::unbounded_channel::<Option<P1A>>();
    let p1a_recv = builder.add_input_from_stream::<_, _, VecHandoff<P1A>, _>(
        "P1A",
        UnboundedReceiverStream::new(p1a_recv),
    );

    // p1b(acceptorID, logSize, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t) # NOTE: logSize necessary because p1b can't send entire log back in 1 msg
    let (p1b_send, p1b_recv) = mpsc::unbounded_channel::<Option<P1B>>();

    // p1bLog(acceptorID, payload, slot, payloadBallotID, payloadBallotNum, ballotID, ballotNum, l, t)
    let (p1blog_send, p1blog_recv) = mpsc::unbounded_channel::<Option<P1BLog>>();

    // p2a(proposerID, payload, slot, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
    let (p2a_send, p2a_recv) = mpsc::unbounded_channel::<Option<P2A>>();
    let p2a_recv = builder.add_input_from_stream::<_, _, VecHandoff<P2A>, _>(
        "P2A",
        UnboundedReceiverStream::new(p2a_recv),
    );

    // p2b(acceptorID, payload, slot, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t)
    let (p2b_send, p2b_recv) = mpsc::unbounded_channel::<Option<P2B>>();

    // Teeing p1a.
    let (p1a_for_ballots_send, p1a_for_ballots_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P1A)>, Option<(usize, P1A)>>("p1a for ballots");
    let (p1a_for_p1b_send, p1a_for_p1b_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P1A)>, Option<(usize, P1A)>>("p1a for p1b");
    let (p1a_for_p1blog_send, p1a_for_p1blog_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P1A)>, Option<(usize, P1A)>>("p1a for p1blog");
    builder.add_subgraph_stratified(
        "tee p1a",
        0,
        p1a_recv
            .flatten()
            .map_with_context(|ctx, p1a| (ctx.current_epoch(), p1a))
            .pull_to_push()
            .map(Some)
            .tee(
                p1a_for_ballots_send,
                builder
                    .start_tee()
                    .tee(p1a_for_p1b_send, p1a_for_p1blog_send),
            ),
    );

    // Teeing p2a.
    let (p2a_for_ballots_send, p2a_for_ballots_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P2A)>, Option<(usize, P2A)>>("p2a for ballots");
    let (p2a_for_log_send, p2a_for_log_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P2A)>, Option<(usize, P2A)>>("p2a for log");
    let (p2a_for_p2b_send, p2a_for_p2b_recv) =
        builder.make_edge::<_, VecHandoff<(usize, P2A)>, Option<(usize, P2A)>>("p2a for p2b");
    builder.add_subgraph_stratified(
        "tee p2a",
        0,
        p2a_recv
            .flatten()
            .map_with_context(|ctx, p2a| (ctx.current_epoch(), p2a))
            .pull_to_push()
            .map(Some)
            .tee(
                p2a_for_ballots_send,
                builder.start_tee().tee(p2a_for_log_send, p2a_for_p2b_send),
            ),
    );

    // IDB: ballots(id, num, l, t) /REMOVE: # Assumes starts with 0,0
    let (ballots_send, ballots_recv) =
        builder.make_edge::<_, VecHandoff<Ballot>, Option<Ballot>>("ballots");
    let (old_ballots_send, old_ballots_recv) =
        builder.make_edge::<_, VecHandoff<Ballot>, Option<Ballot>>("old ballots");
    let (ballots_persist_send, ballots_persist_recv) =
        builder.make_edge::<_, VecHandoff<Ballot>, Option<Ballot>>("ballets persistence rule");

    // Persistence rule
    builder.add_subgraph_stratified(
        "ballot persistence rule",
        999,
        ballots_persist_recv
            .flatten()
            .pull_to_push()
            .map(Some)
            .push_to(old_ballots_send),
    );
    // ######################## reply to p1a
    // ballots(id, num, l, t) :- p1a(_, id, num, l, t)
    // ######################## reply to p2a
    // ballots(id, num, l, t) :- p2a(_, _, _, id, num, l, t)
    let ballots_from_p1a = p1a_for_ballots_recv
        .flatten()
        .map(|(_, P1A { ballot, .. })| ballot);
    let ballots_from_p2a = p2a_for_ballots_recv
        .flatten()
        .map(|(_, P2A { ballot, .. })| ballot);

    builder.add_subgraph_stratified(
        "receive ballots",
        0,
        ballots_from_p1a
            .chain(ballots_from_p2a)
            .chain(old_ballots_recv.flatten())
            .pull_to_push()
            .map(Some)
            .tee(ballots_send, ballots_persist_send),
    );

    // IDB: log(payload, slot, ballotID, ballotNum, l, t)
    let (log_send, log_recv) = builder.make_edge::<_, VecHandoff<Log>, Option<Log>>("log");
    let (old_log_send, old_log_recv) =
        builder.make_edge::<_, VecHandoff<Log>, Option<Log>>("old log persistence");
    // Teeing log.
    let (log_for_log_size_send, log_for_log_size_recv) =
        builder.make_edge::<_, VecHandoff<Log>, Option<Log>>("log for log size");
    let (log_for_log_entry_max_ballot_send, log_for_log_entry_max_ballot_recv) =
        builder.make_edge::<_, VecHandoff<Log>, Option<Log>>("log for log entry max ballot");
    // Persistence.
    let (log_persist_send, log_persist_recv) =
        builder.make_edge::<_, VecHandoff<Log>, Option<Log>>("log persistence rule");

    // Tee log.
    builder.add_subgraph_stratified(
        "tee log",
        0,
        log_recv
            .flatten()
            .chain(old_log_recv.flatten())
            .pull_to_push()
            .map(Some)
            .tee(
                log_for_log_size_send,
                builder
                    .start_tee()
                    .tee(log_for_log_entry_max_ballot_send, log_persist_send),
            ),
    );
    // Persistence rule
    // log(p, slot, ballotID, ballotNum, l, t') :- log(p, slot, ballotID, ballotNum, l, t), succ(t, t')
    builder.add_subgraph_stratified(
        "log persistence rule",
        999,
        log_persist_recv
            .flatten()
            .pull_to_push()
            .map(Some)
            .push_to(old_log_send),
    );

    let (max_ballot_for_log_send, max_ballot_for_log_recv) = builder
        .make_edge::<_, VecHandoff<(usize, Ballot)>, Option<(usize, Ballot)>>("max ballot for log");
    let (max_ballot_for_p1b_send, max_ballot_for_p1b_recv) = builder
        .make_edge::<_, VecHandoff<(usize, Ballot)>, Option<(usize, Ballot)>>("max ballot for p1b");
    let (max_ballot_for_p2b_send, max_ballot_for_p2b_recv) = builder
        .make_edge::<_, VecHandoff<(usize, Ballot)>, Option<(usize, Ballot)>>("max ballot for p2b");

    // MaxBallotNum(max<num>, l, t) :- ballots(_, num, l, t)
    // MaxBallot(max<id>, num, l, t) :- MaxBallotNum(num, l, t), ballots(id, num, l, t)
    // Tees for maxballot.
    builder.add_subgraph_stratified(
        "max ballot",
        1,
        ballots_recv
            .filter_map(|all_ballots_for_this_tick| all_ballots_for_this_tick.into_iter().max())
            .map_with_context(|ctx, max_ballot| (ctx.current_epoch(), max_ballot))
            .pull_to_push()
            .map(Some)
            .tee(
                max_ballot_for_log_send,
                builder
                    .start_tee()
                    .tee(max_ballot_for_p1b_send, max_ballot_for_p2b_send),
            ),
    );

    // LogSize(count<slot>, l, t) :- log(_, slot, _, _, l, t)
    // Counts #log entries in this tick, has to be stratum 1 ... ?
    let log_size = log_for_log_size_recv
        .map(|all_logs| {
            let unique_slots: HashSet<_> = all_logs.into_iter().map(|log| log.slot).collect();
            unique_slots.len() as u32
        })
        .map_with_context(|ctx, size| (ctx.current_epoch(), size));

    // p1b(i, size, ballotID, ballotNum, maxBallotID, maxBallotNum, proposerID, t') :- p1a(proposerID, ballotID, ballotNum, l, t), id(i), LogSize(size, l, t), MaxBallot(maxBallotID, maxBallotNum, l, t), choice(_, t')
    builder.add_subgraph_stratified(
        "p1b",
        1, // Has to be stratum 1 uses max ballot.
        p1a_for_p1b_recv
            .flatten()
            .join(log_size)
            .map(|(epoch, p1a, log_size)| (epoch, (p1a, log_size)))
            .join(max_ballot_for_p1b_recv.flatten())
            .map(move |(_epoch, (p1a, size), max_ballot)| P1B {
                acceptor_id: id,
                log_size: size,
                proposer_ballot: p1a.ballot,
                max_ballot,
            })
            .pull_to_push()
            .map(Some)
            .for_each(move |p1b| p1b_send.send(p1b).expect("`p1b_send` closed!")),
    );

    // LogEntryMaxBallotNum(slot, max<ballotNum>, l, t) :- log(_, slot, _, ballotNum, l, t)
    // LogEntryMaxBallot(slot, max<ballotID>, ballotNum, l, t) :- LogEntryMaxBallotNum(slot, ballotNum, l, t), log(_, slot, ballotID, ballotNum, l, t)
    // output: stream of (slotIdx, max ballot) tuples.
    // Optimized this by keeping the `payload` around. So we do not need to re-join the `payload` below.
    let log_with_max_ballot_by_slot = log_for_log_entry_max_ballot_recv
        .flat_map(|all_logs| {
            let mut log_with_max_ballot_per_slot: HashMap<SlotIdx, Log> = HashMap::new();
            for log in all_logs {
                match log_with_max_ballot_per_slot.entry(log.slot) {
                    std::collections::hash_map::Entry::Occupied(mut old_log_with_max_ballot) => {
                        if log.ballot > old_log_with_max_ballot.get().ballot {
                            old_log_with_max_ballot.insert(log);
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(empty_entry) => {
                        empty_entry.insert(log);
                    }
                }
            }
            log_with_max_ballot_per_slot.into_values()
        })
        .map_with_context(|ctx, x| (ctx.current_epoch(), x));

    // # send back entire log
    // p1bLog(i, payload, slot, payloadBallotID, payloadBallotNum, ballotID, ballotNum, proposerID, t') :-
    //      id(i),
    //      log(payload, slot, payloadBallotID, payloadBallotNum, l, t),      // These two tables
    //      LogEntryMaxBallot(slot, payloadBallotID, payloadBallotNum, l, t), // are combined above as an optimization.
    //      p1a(proposerID, ballotID, ballotNum, l, t),
    //      choice(_, t')
    builder.add_subgraph_stratified(
        "p1blog",
        1,
        p1a_for_p1blog_recv
            .flatten()
            .join(log_with_max_ballot_by_slot)
            .map(move |(_epoch, p1a, log)| P1BLog {
                acceptor_id: id,
                proposer_ballot: p1a.ballot,
                payload: log.payload,
                slot: log.slot,
                payload_ballot: log.ballot,
            })
            .pull_to_push()
            .map(Some)
            .for_each(move |p1blog| p1blog_send.send(p1blog).expect("`p1blog_send` closed!")),
    );

    // log(payload, slot, ballotID, ballotNum, l, t) :- p2a(_, payload, slot, ballotID, ballotNum, l, t), MaxBallot(maxID, maxNum, l, t), ballotGeq(ballotID, ballotNum, maxID, maxNum)
    builder.add_subgraph_stratified(
        "save payload if larger than max ballot",
        1, // Uses max ballot, so use stratum 1 (or greater).
        p2a_for_log_recv
            .flatten()
            .join(max_ballot_for_log_recv.flatten())
            .filter(|(_epoch, p2a, max_ballot)| &p2a.ballot >= max_ballot)
            .map(|(_epoch, p2a, _max_ballot)| Log {
                payload: p2a.payload,
                slot: p2a.slot,
                ballot: p2a.ballot,
            })
            .pull_to_push()
            .map(Some)
            .push_to(log_send),
    );

    // p2b(i, payload, slot, ballotID, ballotNum, maxBallotID, maxBallotNum, proposerID, t') :- p2a(proposerID, payload, slot, ballotID, ballotNum, l, t), id(i), MaxBallot(maxBallotID, maxBallotNum, l, t), choice(_, t')
    builder.add_subgraph_stratified(
        "p2b",
        1, // Uses max ballot, so use stratum 1 (or greater).
        p2a_for_p2b_recv
            .flatten()
            .join(max_ballot_for_p2b_recv.flatten())
            .map(move |(_epoch, p2a, max_ballot)| P2B {
                acceptor_id: id,
                payload: p2a.payload,
                slot: p2a.slot,
                proposer_ballot: p2a.ballot,
                max_ballot,
            })
            .pull_to_push()
            .for_each(move |p2b| p2b_send.send(Some(p2b)).expect("`p2b_send` closed!")),
    );

    let hydroflow = builder.build();
    (
        hydroflow,
        p1a_send,
        p1b_recv,
        p1blog_recv,
        p2a_send,
        p2b_recv,
    )

    // # IDB scratch
    // LogSize(size, l, t)
    // LogEntryMaxBallotNum(slot, ballotNum, l, t)
    // LogEntryMaxBallot(slot, ballotID, ballotNum, l, t)
    // MaxBallotNum(num, l, t)
    // MaxBallot(id, num, l, t)

    // # copied from proposer
    // p1a(proposerID, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
    // p1b(acceptorID, logSize, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t) # NOTE: logSize necessary because p1b can't send entire log back in 1 msg
    // p1bLog(acceptorID, payload, slot, payloadBallotID, payloadBallotNum, ballotID, ballotNum, l, t)
    // p2a(proposerID, payload, slot, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
    // p2b(acceptorID, payload, slot, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t)
    // ######################## end relation definitions

    // ballots(i, n, l, t') :- ballots(i, n, l, t), succ(t, t')
    // <LOG FOR LOG (persistence)>
    // log(p, slot, ballotID, ballotNum, l, t') :- log(p, slot, ballotID, ballotNum, l, t), succ(t, t')

    // ######################## reply to p1a
    // ballots(id, num, l, t) :- p1a(_, id, num, l, t)
    // MaxBallotNum(max<num>, l, t) :- ballots(_, num, l, t)
    // MaxBallot(max<id>, num, l, t) :- MaxBallotNum(num, l, t), ballots(id, num, l, t)
    // <LOG FOR LogSize>
    // LogSize(count<slot>, l, t) :- log(_, slot, _, _, l, t)
    // <MAX_BALLOT_TO_P1B>
    // <p1a for p1b>
    // p1b(i, size, ballotID, ballotNum, maxBallotID, maxBallotNum, proposerID, t') :- p1a(proposerID, ballotID, ballotNum, l, t), id(i), LogSize(size, l, t), MaxBallot(maxBallotID, maxBallotNum, l, t), choice(_, t')

    // <LOG FOR LogEntryMaxBallotNum>
    // LogEntryMaxBallotNum(slot, max<ballotNum>, l, t) :- log(_, slot, _, ballotNum, l, t)
    // <LOG FOR LogEntryMaxBallot>
    // LogEntryMaxBallot(slot, max<ballotID>, ballotNum, l, t) :- LogEntryMaxBallotNum(slot, ballotNum, l, t), log(_, slot, ballotID, ballotNum, l, t)

    // # send back entire log
    // <LOG FOR P1BLog>
    // <p1a for p1blog>
    // p1bLog(i, payload, slot, payloadBallotID, payloadBallotNum, ballotID, ballotNum, proposerID, t') :- id(i), log(payload, slot, payloadBallotID, payloadBallotNum, l, t), LogEntryMaxBallot(slot, payloadBallotID, payloadBallotNum, l, t), p1a(proposerID, ballotID, ballotNum, l, t), choice(_, t')
    // ######################## end reply to p1a

    // ######################## reply to p2a
    // <P2A FOR BALLOTS>
    // ballots(id, num, l, t) :- p2a(_, _, _, id, num, l, t)
    // # Any rule that uses MaxBallot is guaranteed to only run once all ballots have been processed
    // <MAX_BALLOT_TO_LOG>
    // <P2A FOR LOG>
    // ### log(payload, slot, ballotID, ballotNum, l, t) :- p2a(_, payload, slot, ballotID, ballotNum, l, t), MaxBallot(maxID, maxNum, l, t), ballotGeq(ballotID, ballotNum, maxID, maxNum)
    // <MAX_BALLOT_TO_P2B>
    // <P2A FOR P2B>
    // ### p2b(i, payload, slot, ballotID, ballotNum, maxBallotID, maxBallotNum, proposerID, t') :- p2a(proposerID, payload, slot, ballotID, ballotNum, l, t), id(i), MaxBallot(maxBallotID, maxBallotNum, l, t), choice(_, t')
    // ######################## end reply to p2a
}
