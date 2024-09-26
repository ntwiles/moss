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
        .parse("let foo = (x) => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_one_param() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x| => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn non_closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x, y) => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x, y| => { 7; }; foo();")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_one_arg() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x) => { x; }; foo(7);")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_two_args() {
    let parsed = ProgramParser::new()
        .parse("let add = (x, y) => { x + y; }; add(7, 8);")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 15);
}
