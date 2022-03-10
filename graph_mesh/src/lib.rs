use std::cell::Cell;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

type Vid = usize;
type Nid = usize;

///
/// Linked mesh representation:
/// ```text
///         | dests
/// sources | 0 | 1 | 2 | 3 |
/// --------+---+---+---+---+
///       0 |             x |
///       1 | x       x     |
///       2 |             x |
///       3 |     x         |
/// --------+---------------+
///
/// ```
#[derive(Default, Debug)]
pub struct GraphMeshWriter {
    verts: HashMap<Vid, Nid>,
    arena: Vec<Node>,
}

pub struct GraphMesh<'a> {
    verts: HashMap<Vid, Nid>,
    arena: &'a [Node],
}

#[derive(Clone, Debug)]
struct Node {
    src_dst: Option<(usize, usize)>,

    left: Cell<Nid>,
    rght: Cell<Nid>,
    uppp: Cell<Nid>,
    down: Cell<Nid>,
}

impl GraphMeshWriter {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert_vertex(&mut self, vertex: Vid) -> bool {
        match self.verts.entry(vertex) {
            Entry::Occupied(_) => false,
            Entry::Vacant(vacant_entry) => {
                let nid: Nid = self.arena.len();
                vacant_entry.insert(nid);

                let node = Node {
                    src_dst: None,
                    left: Cell::new(nid),
                    rght: Cell::new(nid),
                    uppp: Cell::new(nid),
                    down: Cell::new(nid),
                };

                // println!("v {} nid {}", vertex, nid);

                self.arena.push(node);
                true
            }
        }
    }

    /// Will create duplicate edges.
    pub fn insert_edge(&mut self, src: Vid, dst: Vid) {
        self.insert_vertex(src);
        self.insert_vertex(dst);

        let nid: Nid = self.arena.len();
        self.arena.push(Node {
            src_dst: Some((src, dst)),
            left: Cell::new(nid),
            rght: Cell::new(nid),
            uppp: Cell::new(nid),
            down: Cell::new(nid),
        });
        let new_node = &self.arena[nid];

        let src_node = &self.arena[*self.verts.get(&src).unwrap()];
        new_node.rght.swap(&self.arena[src_node.left.get()].rght);
        new_node.left.swap(&src_node.left);

        let dst_node = &self.arena[*self.verts.get(&dst).unwrap()];
        new_node.down.swap(&self.arena[dst_node.uppp.get()].down);
        new_node.uppp.swap(&dst_node.uppp);

        // println!(
        //     "edge nid {}\nnew {:?}\nsrc {:?}\ndst {:?}",
        //     nid, new_node, src_node, dst_node
        // );
    }

    pub fn finish(&self) -> GraphMesh<'_> {
        GraphMesh {
            verts: self.verts.clone(),
            arena: &*self.arena,
        }
    }
}

impl<'a> GraphMesh<'a> {
    fn iter_row(&self, vid: Vid, mut visit: impl FnMut(&Node, Vid)) {
        let nid = self.verts[&vid];

        let mut node = &self.arena[nid];
        node = &self.arena[node.rght.get()];
        while let Some((src, other_dst)) = node.src_dst {
            assert_eq!(vid, src);
            (visit)(node, other_dst);
            node = &self.arena[node.rght.get()];
        }
    }

    fn iter_col(&self, vid: Vid, mut visit: impl FnMut(&Node, Vid)) {
        let nid = self.verts[&vid];

        let mut node = &self.arena[nid];
        node = &self.arena[node.down.get()];
        while let Some((other_src, dst)) = node.src_dst {
            assert_eq!(vid, dst);
            (visit)(node, other_src);
            node = &self.arena[node.down.get()];
        }
    }

    pub fn contains_vertex(&self, vid: Vid) -> bool {
        self.verts.contains_key(&vid)
    }

    pub fn vertices(
        &self,
    ) -> std::iter::Copied<std::collections::hash_map::Keys<'_, usize, usize>> {
        self.verts.keys().copied()
    }

    pub fn succs(&self, vid: Vid) -> Vec<Vid> {
        let mut out = Vec::new();
        self.iter_row(vid, |_node, dst| out.push(dst));
        out
    }

    pub fn preds(&self, vid: Vid) -> Vec<Vid> {
        let mut out = Vec::new();
        self.iter_col(vid, |_node, src| out.push(src));
        out
    }

    pub fn partition(&mut self, keep: &HashSet<Vid>) -> GraphMesh<'a> {
        let mut output_edges = HashMap::with_capacity(self.verts.len() - keep.len());

        for (&vid, &nid) in self.verts.iter() {
            let retain = if keep.contains(&vid) {
                true
            } else {
                output_edges.insert(vid, nid);
                false
            };

            // Remove row.
            self.iter_row(vid, |node, other_dst| {
                if retain != keep.contains(&other_dst) {
                    self.detach_node(node);
                }
            });
            // Remove col.
            self.iter_col(vid, |node, other_src| {
                if retain != keep.contains(&other_src) {
                    self.detach_node(node);
                }
            });
        }

        self.verts.retain(|v, _| keep.contains(v));

        GraphMesh {
            verts: output_edges,
            arena: self.arena,
        }
    }

    fn detach_node(&self, node: &Node) {
        self.arena[node.left.get()].rght.set(node.rght.get());
        self.arena[node.rght.get()].left.set(node.left.get());
        self.arena[node.uppp.get()].down.set(node.down.get());
        self.arena[node.down.get()].uppp.set(node.uppp.get());
    }
}

#[test]
fn test_basic() {
    let mut graph = GraphMeshWriter::new();
    graph.insert_edge(100, 102);
    graph.insert_edge(101, 102);
    graph.insert_edge(101, 103);
    graph.insert_edge(101, 104);
    graph.insert_edge(102, 101);
    graph.insert_edge(103, 100);
    graph.insert_edge(104, 102);
    graph.insert_edge(104, 103);
    let mut graph = graph.finish();

    assert_eq!(&[102], &*graph.succs(100));
    assert_eq!(&[102, 103, 104], &*graph.succs(101));

    assert_eq!(&[103], &*graph.preds(100));
    assert_eq!(&[100, 101, 104], &*graph.preds(102));

    let keep = [100, 101, 102].into_iter().collect();
    let graph_b = graph.partition(&keep);
    {
        assert_eq!(&[102], &*graph.succs(100));
        assert_eq!(&[102], &*graph.succs(101));

        assert_eq!(&[] as &[usize], &*graph.preds(100));
        assert_eq!(&[100, 101], &*graph.preds(102));
    }
    {
        assert_eq!(&[] as &[usize], &*graph_b.succs(103));
        assert_eq!(&[103], &*graph_b.succs(104));

        assert_eq!(&[104], &*graph_b.preds(103));
        assert_eq!(&[] as &[usize], &*graph_b.preds(104));
    }
}
