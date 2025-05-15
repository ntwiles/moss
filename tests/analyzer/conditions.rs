use moss::{grammar::ProgramParser, test_util};

#[test]
fn if_else_non_matching() {
    let parsed = ProgramParser::new()
        .parse("let foo = if true { 7; } else { false; }; foo;")
        .unwrap();

    test_util::analyze_program(parsed).expect_err("if-else branches must return the same type.");
}
