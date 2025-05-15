use moss::grammar::ProgramParser;

#[test]
fn declare_with_no_return_type() {
    ProgramParser::new()
        .parse("let foo = () => { 7; }; foo();")
        .expect_err("Missing return type");
}
