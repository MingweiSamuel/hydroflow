use hydroflow_macro::hydroflo2_syntax;
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn test_flat_linear() {
    hydroflo2_syntax! {
        source_iter(0..10) -> filter(|n| 0 == n % 2) -> map(|n| 3 * n) -> for_each(|n: u32| println!("{}", n));
    }
}

#[multiplatform_test]
pub fn test_flat_diamond() {
    hydroflo2_syntax! {
        my_tee = source_iter(0..10) -> tee();
        my_tee -> filter(|n| 0 == n % 3) -> map(|n| format!("{}: fizz", n)) -> my_union;
        my_tee -> filter(|n| 0 == n % 5) -> map(|n| format!("{}: buzz", n)) -> my_union;
        my_union = union() -> for_each(|s: String| println!("{}", s));
    }
}

// #[multiplatform_test]
// pub fn test_hydroflo2() {
//     hydroflo2_syntax! {
//         users = source_iter(0..);
//         messages = source_iter(0..);
//         loop {
//             users -> [0]cp;
//             messages -> [1]cp;
//             cp = cross_join() -> for_each(|(user, message)| println!("notify {} of {}", user, message));
//         }
//     }
// }
