use crate::compiled::{Filter, ForEach, Map, Pusherator};
use crate::{tl, tt};
use std::marker::PhantomData;

pub struct PusheratorBuilder<T> {
    _todo: T,
}

pub trait PusheratorBuild {
    type Item;

    type Input;
    type Output;
    fn build(self, input: Self::Input) -> Self::Output;

    // fn map<U, F>(f: F) -> MapBuild<Self::Item, U, O, F, P>
    // where
    //     F: FnMut(Self::Item) -> U,
    // {
    //     unimplemented!();
    // }
}

pub struct InputBuild<T, O>(PhantomData<(T, O)>);
impl<T, O> PusheratorBuild for InputBuild<T, O> {
    type Item = T;

    type Input = O;
    type Output = O;
    fn build(self, input: O) -> O {
        input
    }
}

pub struct MapBuild<T, U, F, O, P>
where
    F: Fn(T) -> U,
    O: Pusherator<Item = U>,
    P: PusheratorBuild<Item = T, Input = Map<T, U, F, O>>,
{
    prev: P,
    f: F,
    _marker: PhantomData<(T, O)>,
}
impl<T, U, F, O, P> PusheratorBuild for MapBuild<T, U, F, O, P>
where
    F: Fn(T) -> U,
    O: Pusherator<Item = U>,
    P: PusheratorBuild<Item = T, Input = Map<T, U, F, O>>,
{
    type Item = U;

    type Input = O;
    type Output = P::Output;
    fn build(self, input: Self::Input) -> Self::Output {
        self.prev.build(Map::new(self.f, input))
    }
}

pub struct FilterBuild<T, F, O, P>
where
    F: Fn(&T) -> bool,
    O: Pusherator<Item = T>,
    P: PusheratorBuild<Item = T, Input = Filter<T, F, O>>,
{
    prev: P,
    f: F,
    _marker: PhantomData<(T, O)>,
}
impl<T, F, O, P> PusheratorBuild for FilterBuild<T, F, O, P>
where
    F: Fn(&T) -> bool,
    O: Pusherator<Item = T>,
    P: PusheratorBuild<Item = T, Input = Filter<T, F, O>>,
{
    type Item = T;

    type Input = O;
    type Output = P::Output;
    fn build(self, input: Self::Input) -> Self::Output {
        self.prev.build(Filter::new(self.f, input))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_builder() {
        let pb = InputBuild::<usize, _>(PhantomData);
        let pb = FilterBuild {
            prev: pb,
            f: |&x| 0 == x % 2,
            _marker: PhantomData,
        };
        let pb = MapBuild {
            prev: pb,
            f: |x| x * x,
            _marker: PhantomData,
        };

        let mut z = pb.build(ForEach::new(|x| println!("val: {}", x)));

        for x in 0..10 {
            z.give(x);
        }
    }
}

// pub struct ForEachBuild<T, F, P>
// where
//     F: FnMut(T),
//     P: PusheratorBuild<Item = T>,
// {
//     prev: P,
//     f: F,
//     _marker: PhantomData<T>,
// }
// impl<T, F, P> PusheratorBuild for ForEachBuild<T, F, P>
// where
//     F: FnMut(T),
//     P: PusheratorBuild<Item = T>,
// {
//     type Item = !;

//     type Input = O;
//     type Output = Filter<T, F, O>;
//     fn build(self, input: Self::Input) -> Self::Output {
//         Self::Output::new(self.f, input)
//     }
// }

//////////

// pub struct PusheratorBuilder<B, O>
// where
//     B: PusheratorBuild<O>,
// {
//     build: B,
//     _phantom: PhantomData<fn(O)>,
// }

// pub trait PusheratorBuild<O> {
//     type OutputChain;

//     type Output: Pusherator;
//     fn build(self, out: O) -> Self::Output;
// }

// pub struct MapBuild<T, U, F>
// where
//     F: Fn(T) -> U,
// {
//     f: F,
//     _marker: PhantomData<T>,
// }
// impl<T, U, F, O> PusheratorBuild<O> for MapBuild<T, U, F>
// where
//     F: Fn(T) -> U,
//     O: Pusherator<Item = U>,
// {
//     type OutputChain = !; // TODO(mingwei)!

//     type Output = Map<T, U, F, O>;
//     fn build(self, out: O) -> Self::Output {
//         Map::new(self.f, out)
//     }
// }

// // pub trait Append<T> {
// //     type Append;
// // }
// // impl<A, B, T> Append<T> for (A, B)
// // where
// //     B: Append<T>,
// // {
// //     type Append = (A, B::Append);
// // }
// // impl<T> Append<T> for () {
// //     type Append = (T, ());
// // }

// // pub trait Reverse {
// //     type Reverse;
// // }

// // impl<A, B> Reverse for (A, B)
// // where
// //     B: Reverse,
// //     B::Reverse: Append<A>,
// // {
// //     type Reverse = <B::Reverse as Append<A>>::Append;
// // }
// // impl Reverse for () {
// //     type Reverse = ();
// // }

// // struct A0();
// // struct B0();
// // struct C0();

// // type MyList1 = tt!(A0, B0, C0);
// // type MyList2 = <MyList1 as Reverse>::Reverse;

// // fn x() {
// //     let z: MyList2 = tl!(C0(), B0(), A0());
// // }

// // use crate::compiled::Pusherator;

// // pub trait PusheratorBuild<T> {
// //     type Build: Pusherator<Item = T>;
// //     fn build(self) -> Self::Build;

// //     fn map<F, U>(self, f: F) -> MapBuild<F, T, U, Self>
// //     where
// //         Self: Sized,
// //         F: Fn(T) -> U,
// //     {
// //         MapBuild::new(self, f)
// //     }
// // }

// // pub struct MapBuild<F, T, U, B>
// // where
// //     F: FnMut(T) -> U,
// //     B: PusheratorBuild<T>,
// // {
// //     build: B,
// //     f: F,
// //     _phantom: PhantomData<fn(T) -> U>,
// // }
// // impl<F, T, U, B> MapBuild<F, T, U, B>
// // where
// //     F: FnMut(T) -> U,
// //     B: PusheratorBuild<T>,
// // {
// //     pub fn new(build: B, f: F) -> Self {
// //         Self {
// //             build,
// //             f,
// //             _phantom: PhantomData,
// //         }
// //     }
// // }
// // impl<F, T, U, B> PusheratorBuild<T> for MapBuild<F, T, U, B>
// // where
// //     F: FnMut(T) -> U,
// //     B: PusheratorBuild<T>,
// // {
// //     type Build =
// // }

// // pub struct Builder<T> {
// //     _phantom: PhantomData<*mut T>,
// // }

// // impl<T> Builder<T> {}
