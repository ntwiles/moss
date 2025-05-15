use moss::analyzer;
use moss::builtins::get_builtin_bindings;
use moss::grammar::ProgramParser;
use moss::test_util::exec_program;

#[test]
fn assignment_basic() {
    let parsed = ProgramParser::new().parse("let foo = 2 + 5; foo;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn assignment_operated_on() {
    let parsed = ProgramParser::new()
        .parse("let foo = 2 + 5; foo + 3;")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 10);
}

#[test]
fn assignment_function() {
    let parsed = ProgramParser::new()
        .parse("let foo = ||: Int => { 5; };")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtin_bindings()).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_void(), ());
}
