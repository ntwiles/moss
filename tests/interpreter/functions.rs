use moss::grammar::ProgramParser;
use moss::test_util;

#[test]
fn non_closure_no_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = (): Int => { 7; }; foo();")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_no_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = ||: Int => { 7; }; foo();")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn non_closure_one_param() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int): Int => { 7; }; foo(0);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_one_param() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x: Int|: Int => { 7; }; foo(0);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn non_closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int, y: Int): Int => { 7; }; foo(0, 0);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn closure_two_params() {
    let parsed = ProgramParser::new()
        .parse("let foo = |x: Int, y: Int|: Int => { 7; }; foo(0, 0);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_one_arg() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int): Int => { x; }; foo(7);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn call_two_args() {
    let parsed = ProgramParser::new()
        .parse("let add = (x: Int, y: Int): Int => { x + y; }; add(7, 8);")
        .unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 15);
}

#[test]
fn call_with_composition() {
    let code = r"
    let add = (a: Int, b: Int): Int => {
        a + b;
    };

    let sub = (a: Int, b: Int): Int => {
        a - b;
    };

    sub(add(3, 2), 1);
    ";

    let parsed = ProgramParser::new().parse(code).unwrap();
    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 4);
}
