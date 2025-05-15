use moss::analyzer;
use moss::builtins::get_builtin_bindings;
use moss::grammar::ProgramParser;
use moss::test_util::exec_program;

#[test]
fn if_else_basic_true() {
    let parsed = ProgramParser::new()
        .parse("if true { 7; } else { 8; };")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn if_else_basic_false() {
    let parsed = ProgramParser::new()
        .parse("if false { 7; } else { 8; };")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 8);
}

#[test]
fn if_else_assign() {
    let parsed = ProgramParser::new()
        .parse("let foo = if true { 7; } else { 8; }; foo;")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn if_else_non_matching() {
    let parsed = ProgramParser::new()
        .parse("let foo = if true { 7; } else { false; }; foo;")
        .unwrap();

    analyzer::analyze_program(parsed, get_builtin_bindings())
        .expect_err("if-else branches must return the same type.");
}
