use super::PullSurface;

use crate::builder::build::pull_filter::FilterPullBuild;

pub struct MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
{
    prev: Prev,
    func: Func,
}
impl<Prev, Func> MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

impl<Prev, Func> PullSurface for MapPullSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(&Prev::ItemOut) -> bool,
{
    type InputHandoffs = Prev::InputHandoffs;

    type ItemOut = Prev::ItemOut;

    type Connect = Prev::Connect;
    type Build = FilterPullBuild<Prev::Build, Func>;

    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let (connect, build) = self.prev.into_parts();
        let build = FilterPullBuild::new(build, self.func);
        (connect, build)
    }
}
