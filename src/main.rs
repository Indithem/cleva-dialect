use tree_sitter::Parser;
extern crate tree_sitter_cleva;


fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_cleva::language()).unwrap();
    let code = r#"
int double(int x) {
    return x * 2;
}
"#;
    let tree = parser.parse(code, None);
    println!("{:?}", tree);
}
