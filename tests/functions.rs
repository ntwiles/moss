use moss::analyzer;
use moss::grammar::ProgramParser;
use moss::interpretor;

#[test]
fn non_closure_no_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = () => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_no_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = || => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn non_closure_one_param() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int) => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_one_param() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x: Int| => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn non_closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int, y: Int) => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x: Int, y: Int| => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_one_arg() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int) => { x; }; foo(7);")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_two_args() {
    let parsed = ProgramParser::new()
        .parse("let add = (x: Int, y: Int) => { x + y; }; add(7, 8);")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 15);
}

// TODO: Errors when args don't match params.

#[test]
fn call_wrong_arg() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int) => { x; }; foo(false);")
        .unwrap();

    analyzer::analyze_program(parsed).expect_err("foo expects 1 argument, got 2.");
}

#[test]
fn call_too_few_args() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int, y: Int) => { x + y; }; foo(7);")
        .unwrap();

    analyzer::analyze_program(parsed).expect_err("foo expects 2 arguments, got 1.");
}
