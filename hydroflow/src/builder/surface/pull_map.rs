use super::PullSurface;

use crate::builder::build::pull_map::MapPullBuild;

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
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

impl<Prev, Func, Out> PullSurface for MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(Prev::ItemOut) -> Out,
{
    type InputHandoffs = Prev::InputHandoffs;

    type ItemOut = Out;

    type Connect = Prev::Connect;
    type Build = MapPullBuild<Prev::Build, Func>;

    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let (connect, build) = self.prev.into_parts();
        let build = MapPullBuild::new(build, self.func);
        (connect, build)
    }
}
