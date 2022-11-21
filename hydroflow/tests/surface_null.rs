// TODO(mingwei): Need rust-analyzer support
#![allow(clippy::uninlined_format_args)]

use hydroflow::hydroflow_syntax;

#[test]
pub fn test_null_as_input() {
    let mut df = hydroflow_syntax! {
        generator = null();
        generator[0] -> for_each(|_: ()| println!("0: if you see this something is wrong"));
        generator[1] -> for_each(|_: ()| println!("1: if you see this something is wrong"));
    };
    df.run_available();
}
