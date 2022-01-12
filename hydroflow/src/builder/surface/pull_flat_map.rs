use super::PullSurface;

use crate::builder::build::pull_flat_map::FlatMapPullBuild;

pub struct MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
{
    prev: Prev,
    func: Func,
}
impl<Prev, Func, Out> MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(Prev::ItemOut) -> Out,
    Out: IntoIterator,
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

impl<Prev, Func, Out> PullSurface for MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(Prev::ItemOut) -> Out,
    Out: IntoIterator,
{
    type InputHandoffs = Prev::InputHandoffs;

    type ItemOut = Out::Item;

    type Connect = Prev::Connect;
    type Build = FlatMapPullBuild<Prev::Build, Func>;

    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let (connect, build) = self.prev.into_parts();
        let build = FlatMapPullBuild::new(build, self.func);
        (connect, build)
    }
}
