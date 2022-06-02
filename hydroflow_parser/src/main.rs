use tree_sitter::Parser;

fn main() {
    let code = r#"
    function double(x) {
        return x * 2;
    }
"#;
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_javascript::language())
        .expect("Error loading JavaScript grammar");
    let parsed = parser.parse(code, None);
    println!("{:#?}", parsed);
}
