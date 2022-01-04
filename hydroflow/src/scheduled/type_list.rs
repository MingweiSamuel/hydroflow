pub trait TypeList {}

impl TypeList for () {}
impl<X, T> TypeList for (X, T) where T: TypeList {}

pub trait Extend<U>: TypeList
where
    U: TypeList,
{
    type Extended: TypeList;
    fn extend(self, input: U) -> Self::Extended;
}

impl<X, T, U> Extend<U> for (X, T)
where
    T: TypeList + Extend<U>,
    U: TypeList,
{
    type Extended = (X, <T as Extend<U>>::Extended);
    fn extend(self, input: U) -> Self::Extended {
        let (x, t) = self;
        (x, t.extend(input))
    }
}
impl<U> Extend<U> for ()
where
    U: TypeList,
{
    type Extended = U;
    fn extend(self, input: U) -> Self::Extended {
        input
    }
}



pub trait SplitPrefix<U>: TypeList
where
    U: TypeList,
{
    type Suffix: TypeList;
    fn split(self) -> (U, Self::Suffix);
}
impl<X, T, U> SplitPrefix<(X, U)> for (X, T)
where
    U: TypeList,
    T: SplitPrefix<U>
{
    type Suffix = <T as SplitPrefix<U>>::Suffix;
    fn split(self) -> ((X, U), Self::Suffix) {
        let (x, t) = self;
        let (t, u) = t.split();
        ((x, t), u)
    }
}
impl<T> SplitPrefix<()> for T
where
    T: TypeList,
{
    type Suffix = T;
    fn split(self) -> ((), Self::Suffix) {
        ((), self)
    }
}

#[cfg(test)]
mod test {
    use crate::{tt, tl};

    use super::*;

    type MyList = tt!(u8, u16, u32, u64);
    type MyPrefix = tt!(u8, u16);

    type MySuffix = <MyList as SplitPrefix<MyPrefix>>::Suffix;

    const _: MySuffix = tl!(0_u32, 0_u64);
}



// pub trait TypeTree {
//     type Flatten: TypeList;
//     fn unflatten(flat: Self::Flatten) -> Self;
// }
// impl<L> TypeTree for L
// where
//     L: TypeTreeList,
// {
//     type Flatten = L::Flatten;
//     fn unflatten(flat: Self::Flatten) -> Self {
//         flat.unflatten()
//     }
// }
// impl<X> TypeTree for (X,) {
//     type Flatten = tt!(X);
//     fn unflatten(flat: Self::Flatten) -> Self;
// }

// pub trait TypeTreeList: TypeList {
//     type Flatten: TypeList;
//     fn unflatten(flat: Self::Flatten) -> Self;
// }
// impl<X, T> TypeTreeList for (X, T)
// where
//     X: TypeTree,
//     X::Flatten: Extend<T::Flatten>,
//     T: TypeTreeList,
// {
//     type Flatten = <X::Flatten as Extend<T::Flatten>>::Output;
//     fn unflatten(flat: Self::Flatten) -> Self {}
// }
// impl TypeTreeList for () {
//     type Flatten = ();
//     fn unflatten(flat: Self::Flatten) -> Self {
//         flat
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::tl;

//     type MyBranchA = tt!((u32,), (u64,));
//     type MyBranchB = tt!((i32,), (i64,));
//     type MyTree = tt!(MyBranchA, MyBranchB, (u8,));

//     type MyList = <MyTree as TypeTreeList>::Flatten;

//     #[allow(dead_code)]
//     const _: MyList = tl!(0_u32, 0_u64, 0_i32, 0_i64, 0_u8);
// }
