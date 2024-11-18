use graze::{
    ast::{parse_file, Program},
    output::svg::SvgOutput,
    runtime::Runtime,
    token::StringTokenizer,
};

fn main() {
    let program = r#"
        pnt2 10 10; pnt2 20 20;line
        "#;
    let mut tokens = StringTokenizer::new(&program);
    let ast = parse_file(&mut tokens);
    let mut rt = Runtime::<SvgOutput>::default();
    rt.execute(ast.unwrap()).unwrap();
    rt.finish();
}
