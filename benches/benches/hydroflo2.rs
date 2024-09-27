use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::LazyLock;

use criterion::{criterion_group, criterion_main, Criterion};
use hydroflow::itertools::{chain, Itertools};
use nameof::name_of;

/// my_tee = source_iter(0..10) -> tee();
/// a = my_tee -> filter(|n| 0 == n % 3) -> map(|n| format!("{}: fizz", n)) -> my_union;
/// b = my_tee -> filter(|n| 0 == n % 5) -> map(|n| format!("{}: buzz", n)) -> my_union;
/// my_union = union() -> for_each(|s: String| println!("{}", s));

static WORDS: LazyLock<String> = LazyLock::new(|| {
    let mut path = PathBuf::new();
    path.push(std::env::current_dir().unwrap());
    path.pop();
    path.push(file!());
    path.pop();
    path.push("words_alpha.txt");
    println!("{:?}", path);
    std::fs::read_to_string(path).unwrap()
});
fn words() -> impl Iterator<Item = String> + Clone {
    WORDS
        .lines()
        .filter(|&s| 0 != hash_code(s) % 2)
        .map(|s| s.to_owned())
}
fn hash_code(s: &str) -> u32 {
    s.bytes().fold(0, |n, c| (n * 31).wrapping_add(c as u32))
}

fn hydroflo2_diamond_forloop(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_forloop), |b| {
        b.iter(|| {
            let mut c = 0;
            for s in words() {
                let a = { [format!("hi {}", s), format!("bye {}", s)] };
                let b = {
                    if 0 == s.len() % 5 {
                        Some(s)
                    } else {
                        None
                    }
                };
                for s in a.into_iter().chain(b) {
                    c += s.len();
                }
            }
            assert_eq!(5_123_595, c);
        })
    });
}
fn hydroflo2_diamond_iter_clone_chain(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_clone_chain), |b| {
        b.iter(|| {
            let i = words();
            let a = i
                .clone()
                .flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let b = i.filter(|s| 0 == s.len() % 5);
            let n = a.chain(b).fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}
fn hydroflo2_diamond_iter_clone_interleave(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_clone_interleave), |b| {
        b.iter(|| {
            let i = words();
            let a = i
                .clone()
                .flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let b = i.filter(|s| 0 == s.len() % 5);
            let n = a.interleave(b).fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}
fn hydroflo2_diamond_iter_buffer_chain(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_buffer_chain), |b| {
        b.iter(|| {
            let i = words();
            let v = i.collect::<Vec<_>>();
            let a = v
                .iter()
                .cloned()
                .flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let b = v.iter().cloned().filter(|s| 0 == s.len() % 5);
            let n = a.chain(b).fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}
fn hydroflo2_diamond_iter_tee_chain(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_tee_chain), |b| {
        b.iter(|| {
            let i = words();
            let (a, b) = i.tee();
            let a = a.flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let b = b.filter(|s| 0 == s.len() % 5);
            let n = a.chain(b).fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}

fn hydroflo2_diamond_iter_tee_interleave(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_tee_interleave), |b| {
        b.iter(|| {
            let i = words();
            let (a, b) = i.tee();
            let a = a.flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let b = b.filter(|s| 0 == s.len() % 5);
            let c = a.interleave(b);
            let n = c.fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}

fn hydroflo2_diamond_iter_buffer_one(c: &mut Criterion) {
    let _ = *WORDS;

    c.bench_function(name_of!(hydroflo2_diamond_iter_buffer_one), |b| {
        b.iter(|| {
            let i = words();
            let mut buffer = RefCell::new(Vec::new());
            let a = i
                .inspect(|s| buffer.borrow_mut().push(s.clone()))
                .flat_map(|s| [format!("hi {}", s), format!("bye {}", s)]);
            let c = a.chain_with(|| buffer.borrow_mut().drain(..).filter(|s| 0 == s.len() % 5));
            let n = c.fold(0, |n, s| n + s.len());
            assert_eq!(5_123_595, n);
        })
    });
}

// // enum Diamond<Iter, Item> {
// //     Filling { iter: Iter, buffer: Vec<Item> },
// //     Draining { drain: std::vec::IntoIter<Item> },
// // }
// fn diamond<Iter, In, Out, A, B>(
//     iter: Iter,
//     fn_a: impl FnOnce(Fill<'_, Iter::IntoIter, Iter::Item>) -> A,
//     fn_b: impl FnOnce(std::vec::IntoIter<In>) -> B,
// ) where
//     Iter: IntoIterator<Item = In>,
//     A: IntoIterator<Item = Out>,
//     B: IntoIterator<Item = Out>,
// {
//     let iter = iter.into_iter();
//     let mut buffer = Vec::with_capacity(iter.size_hint().0);
//     let a = Fill {
//         iter,
//         buffer: &mut buffer,
//     };
//     let a = (fn_a)(a);
// }

// // enum Diamond

// // struct Fill<'a, Iter, Item> {
// //     iter: Iter,
// //     buffer: &'a mut Vec<Item>,
// // }
// // impl<'a, Iter, Item> Iterator for Fill<'a, Iter, Item>
// // where
// //     Iter: Iterator<Item = Item>,
// //     Item: Clone,
// // {
// //     type Item = Item;

// //     fn next(&mut self) -> Option<Self::Item> {
// //         if let Some(item) = self.iter.next() {
// //             self.buffer.push(item.clone());
// //             Some(item)
// //         } else {
// //             None
// //         }
// //     }

// //     fn size_hint(&self) -> (usize, Option<usize>) {
// //         self.iter.size_hint()
// //     }
// // }

criterion_group!(
    hydroflo2,
    hydroflo2_diamond_forloop,
    hydroflo2_diamond_iter_clone_chain,
    hydroflo2_diamond_iter_clone_interleave,
    hydroflo2_diamond_iter_buffer_chain,
    hydroflo2_diamond_iter_tee_chain,
    hydroflo2_diamond_iter_tee_interleave,
    hydroflo2_diamond_iter_buffer_one,
);
criterion_main!(hydroflo2);

trait IteratorExt: Iterator {
    fn chain_with<F, I>(self, f: F) -> ChainWith<Self, F, I::IntoIter>
    where
        Self: Sized,
        F: FnOnce() -> I,
        I: IntoIterator<Item = Self::Item>,
    {
        ChainWith {
            base: self,
            factory: Some(f),
            iterator: None,
        }
    }
}

impl<I: Iterator> IteratorExt for I {}

struct ChainWith<B, F, I> {
    base: B,
    factory: Option<F>,
    iterator: Option<I>,
}

impl<B, F, I> Iterator for ChainWith<B, F, I::IntoIter>
where
    B: Iterator,
    F: FnOnce() -> I,
    I: IntoIterator<Item = B::Item>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.base.next() {
            return Some(b);
        }

        // Exhausted the first, generate the second
        if let Some(f) = self.factory.take() {
            self.iterator = Some(f().into_iter());
        }

        self.iterator
            .as_mut()
            .expect("There must be an iterator")
            .next()
    }
}
