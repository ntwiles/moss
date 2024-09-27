use moss::analyzer;
use moss::grammar::ProgramParser;
use moss::interpretor;

#[test]
fn assignment_basic() {
    let parsed = ProgramParser::new().parse("let foo = 2 + 5; foo;").unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn assignment_operated_on() {
    let parsed = ProgramParser::new()
        .parse("let foo = 2 + 5; foo + 3;")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 10);
}

#[test]
fn assignment_function() {
    let parsed = ProgramParser::new()
        .parse("let foo = || => { 5; };")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_program(analyzed).unwrap();

    assert_eq!(result.unwrap_void(), ());
}
