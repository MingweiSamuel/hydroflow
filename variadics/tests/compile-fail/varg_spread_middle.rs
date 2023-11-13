use variadics::*;

fn main() {
    let varg!(a, ...b, c) = var!(1, 2.0, "three", false);
}