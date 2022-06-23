use hydroflow::hydroflow_parser;

#[test]
pub fn test_parser_basic() {
    println!("DONE");

    // hydroflow_parser! {
    //     edges_input = (input() ->);

    //     init_vertex = (seed([0]) ->);
    //     // loop_vertices = (->);
    //     out_vertices = (-> for_each(|x| println!("Reached: {}", x)));

    //     reached_vertices = (merge[init_vertex, loop_vertices] -> map(|v| (v, ())));

    //     (join[reached_vertices, edges_input] -> map(|(_src, ((), dst))| dst) -> dedup() -> tee[loop_vertices, out_vertices]);

    //     // x = (a -> b() -> c() -> (a -> b -> c) -> p);
    //     // b = (a -> b() -> c() -> (a -> b -> c) -> p);
    //     // x = (a -> b() -> c() -> (a -> b -> c) -> p);
    //     // x = (a -> b() -> c() -> (a -> b -> c) -> p);
    // }

    hydroflow_parser! {
        reached_vertices = (merge() -> map(|v| (v, ())));
        (seed([0]) -> [0]reached_vertices);

        my_join = (join() -> map(|(_src, ((), dst))| dst) -> dedup() -> tee());
        (reached_vertices -> [0]my_join);
        (input(/*(v, v) edges*/) -> [1]my_join);

        (my_join[0] -> [1]reached_vertices);
        (my_join[1] -> for_each(|x| println!("Reached: {}", x)));
    }

    hydroflow_parser! {
        shuffle = (merge() -> tee());
        (shuffle[0] -> [0]shuffle);
        (shuffle[1] -> [1]shuffle);
        (shuffle[2] -> [2]shuffle);
        (shuffle[3] -> [3]shuffle);
    }

    hydroflow_parser! {
        x = (map(a) -> map(b));
        (x -> x);
    }
}
