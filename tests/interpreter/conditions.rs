use moss::grammar::ProgramParser;
use moss::test_util;

#[test]
fn if_else_basic_true() {
    let parsed = ProgramParser::new()
        .parse("if true { 7; } else { 8; };")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn if_else_basic_false() {
    let parsed = ProgramParser::new()
        .parse("if false { 7; } else { 8; };")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 8);
}

#[test]
fn if_else_assign() {
    let parsed = ProgramParser::new()
        .parse("let foo = if true { 7; } else { 8; }; foo;")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}
