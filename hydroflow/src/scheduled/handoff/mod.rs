// mod tee;
// pub use tee::TeeingHandoff;

mod vector;
pub use vector::VecHandoff;

pub trait TryCanReceive<T>: Handoff {
    fn try_give(state: &mut Self::State, item: T) -> Result<T, T>;
}
pub trait CanReceive<T>: Handoff {
    fn give(state: &mut Self::State, item: T) -> T;
}

pub trait Handoff {
    type State;

    // Scheduling metadata.
    // TODO(justin): more fine-grained info here.
    fn is_bottom(state: &Self::State) -> bool;

    type Inner;
    fn take_inner(state: &mut Self::State) -> Self::Inner;

    fn give<T>(state: &mut Self::State, item: T) -> T
    where
        Self: CanReceive<T>,
    {
        <Self as CanReceive<T>>::give(state, item)
    }

    fn try_give<T>(state: &mut Self::State, item: T) -> Result<T, T>
    where
        Self: TryCanReceive<T>,
    {
        <Self as TryCanReceive<T>>::try_give(state, item)
    }
}
