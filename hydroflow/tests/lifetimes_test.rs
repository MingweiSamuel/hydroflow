use std::ops::Deref;

struct Context {
    val: String,
}
impl Context {
    fn get_ref_1(&self) -> &str {
        self.val.deref()
    }

    fn get_ref_2(&self) -> &'_ str {
        self.val.deref()
    }
}

fn chars_iter_1<'a, I>(ctx: &'a Context, iter: I) -> impl 'a + Iterator<Item = char>
where
    I: 'a + Iterator<Item = char>,
{
    ctx.get_ref_1().chars().chain(iter)
}
fn chars_iter_2<'a, I>(ctx: &'a Context, iter: I) -> impl 'a + Iterator<Item = char>
where
    I: 'a + Iterator<Item = char>,
{
    ctx.get_ref_2().chars().chain(iter)
}
