macro_rules! compile_fail {
    (
        $( $attr:meta )*
        $v:vis fn $i:ident() {
            $( $t:tt )*
        }
    ) => {
        #[allow(dead_code)]
        #[doc = "```compile_fail"]
        #[doc = stringify!( $( $t )* )]
        #[doc = "```"]
        $( $attr )*
        $v fn $i() {}
    };
}

compile_fail! {
    #[test]
    pub fn test_fail() {
        hydroflow_macro! {

        }
    }
}
