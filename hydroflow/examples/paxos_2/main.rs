pub mod acceptor;

use hydroflow::builder::prelude::*;

use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio_stream::wrappers::UnboundedReceiverStream;

fn main() {
    fn receive_all<T>(mut recv: UnboundedReceiver<T>) -> Vec<T> {
        let mut out = Vec::new();
        while let Ok(item) = recv.try_recv() {
            out.push(item);
        }
        out
    }

    let (mut hydroflow, p1a_send, p1b_recv, p1blog_recv, p2a_send, p2b_recv) =
        acceptor::acceptor(420);

    println!("{}", hydroflow.generate_mermaid());

    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();

    p1a_send
        .send(Some(P1A {
            proposer_id: 2,
            ballot: Ballot { id: 2, num: 2 },
        }))
        .unwrap();

    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();

    p1a_send
        .send(Some(P1A {
            proposer_id: 3,
            ballot: Ballot { id: 3, num: 1 },
        }))
        .unwrap();

    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();

    p1a_send
        .send(Some(P1A {
            proposer_id: 1,
            ballot: Ballot { id: 1, num: 2 },
        }))
        .unwrap();

    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();
    hydroflow.next_stratum();
    hydroflow.tick_stratum();

    println!("{:#?}", receive_all(p1b_recv));
    println!("{:#?}", receive_all(p1blog_recv));
    println!("{:#?}", receive_all(p2b_recv));
}

pub type AcceptorId = u32;
pub type ProposerId = u32;
pub type BallotNum = u32;
pub type SlotIdx = u32;

/// ballots(id, num, l, t) # Assumes starts with 0,0
#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Ballot {
    // THESE FIELDS ARE SWAPPED! BE CAREFUL :)
    // ballotGeq(id1, num1, id2, num2) # true if num1 >= num2 or (num1 = num2 and id1 >= id2)
    // MaxBallotNum(max<num>, l, t) :- ballots(_, num, l, t)
    // MaxBallot(max<id>, num, l, t) :- MaxBallotNum(num, l, t), ballots(id, num, l, t)
    pub num: BallotNum, // Must sort ballot number first, then proposer id.
    pub id: ProposerId,
}

/// log(payload, slot, ballotID, ballotNum, l, t)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Log {
    pub payload: String,
    pub slot: SlotIdx,
    pub ballot: Ballot,
}

/// p1a(proposerID, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct P1A {
    pub proposer_id: ProposerId,
    pub ballot: Ballot,
}

/// p1b(acceptorID, logSize, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t)
#[derive(Clone, Copy, Debug)]
pub struct P1B {
    pub acceptor_id: AcceptorId,
    pub log_size: u32,
    pub proposer_ballot: Ballot,
    pub max_ballot: Ballot,
}

/// p1bLog(acceptorID, payload, slot, payloadBallotID, payloadBallotNum, ballotID, ballotNum, l, t)
#[derive(Clone, Debug)]
pub struct P1BLog {
    pub acceptor_id: AcceptorId,
    pub payload: String,
    pub slot: SlotIdx,
    pub payload_ballot: Ballot,
    pub proposer_ballot: Ballot,
}

/// p2a(proposerID, payload, slot, ballotID, ballotNum, l, t) # proposerID is the location of the proposer
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct P2A {
    #[allow(dead_code)] // Determines msg destination.
    proposer_id: ProposerId,
    payload: String,
    slot: SlotIdx,
    ballot: Ballot,
}

/// p2b(acceptorID, payload, slot, ballotID, ballotNum, maxBallotID, maxBallotNum, l, t)
#[derive(Clone, Debug)]
pub struct P2B {
    acceptor_id: AcceptorId,
    payload: String,
    slot: SlotIdx,
    proposer_ballot: Ballot,
    max_ballot: Ballot,
}
