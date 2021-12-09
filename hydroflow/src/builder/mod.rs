use std::marker::PhantomData;

// use crate::compiled::Pusherator;

// pub trait PusheratorBuild<T> {
//     type Build: Pusherator<Item = T>;
//     fn build(self) -> Self::Build;

//     fn map<F, U>(self, f: F) -> MapBuild<F, T, U, Self>
//     where
//         Self: Sized,
//         F: Fn(T) -> U,
//     {
//         MapBuild::new(self, f)
//     }
// }

// pub struct MapBuild<F, T, U, B>
// where
//     F: FnMut(T) -> U,
//     B: PusheratorBuild<T>,
// {
//     build: B,
//     f: F,
//     _phantom: PhantomData<fn(T) -> U>,
// }
// impl<F, T, U, B> MapBuild<F, T, U, B>
// where
//     F: FnMut(T) -> U,
//     B: PusheratorBuild<T>,
// {
//     pub fn new(build: B, f: F) -> Self {
//         Self {
//             build,
//             f,
//             _phantom: PhantomData,
//         }
//     }
// }
// impl<F, T, U, B> PusheratorBuild<T> for MapBuild<F, T, U, B>
// where
//     F: FnMut(T) -> U,
//     B: PusheratorBuild<T>,
// {
//     type Build = 
// }

// pub struct Builder<T> {
//     _phantom: PhantomData<*mut T>,
// }

// impl<T> Builder<T> {}
