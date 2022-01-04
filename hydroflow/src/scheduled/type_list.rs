pub trait TypeList {}

impl TypeList for () {}
impl<X, T> TypeList for (X, T) where T: TypeList {}

pub trait Extend<U>: TypeList
where
    U: TypeList,
{
    type Output;
}

impl<X, T, U> Extend<U> for (X, T)
where
    T: TypeList + Extend<U>,
    U: TypeList,
{
    type Output = (X, <T as Extend<U>>::Output);
}
impl<U> Extend<U> for ()
where
    U: TypeList,
{
    type Output = U;
}
