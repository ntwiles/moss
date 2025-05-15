use moss::grammar::ProgramParser;
use moss::test_util::{analyze_program, exec_program};

#[test]
fn assignment_basic() {
    let parsed = ProgramParser::new().parse("let foo = 2 + 5; foo;").unwrap();

    let analyzed = analyze_program(parsed).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn assignment_operated_on() {
    let parsed = ProgramParser::new()
        .parse("let foo = 2 + 5; foo + 3;")
        .unwrap();

    let analyzed = analyze_program(parsed).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 10);
}

#[test]
fn assignment_function() {
    let parsed = ProgramParser::new()
        .parse("let foo = ||: Int => { 5; };")
        .unwrap();

    let analyzed = analyze_program(parsed).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_void(), ());
}
