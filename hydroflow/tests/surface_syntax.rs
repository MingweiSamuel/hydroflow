use hydroflow::hydroflow_parser;

#[test]
pub fn test_parser_basic() {
    hydroflow_parser! {
        a()
    }
}
