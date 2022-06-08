use hydroflow::hydroflow_parser;

#[test]
pub fn test_parser_basic() {
    hydroflow_parser! {
        edges_input = input();

        reached_vertices = seed([0]);

        join = join();
        (edges_input -> join);


        // x = (a -> b() -> c() -> (a -> b -> c) -> p);
        // b = (a -> b() -> c() -> (a -> b -> c) -> p);
        // x = (a -> b() -> c() -> (a -> b -> c) -> p);
        // x = (a -> b() -> c() -> (a -> b -> c) -> p);
    }
}
