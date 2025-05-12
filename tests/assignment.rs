use moss::analyzer;
use moss::grammar::ProgramParser;
use moss::interpretor;
use moss::shared::builtins::get_builtins;

#[test]
fn assignment_basic() {
    let parsed = ProgramParser::new().parse("let foo = 2 + 5; foo;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn assignment_operated_on() {
    let parsed = ProgramParser::new()
        .parse("let foo = 2 + 5; foo + 3;")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_int(), 10);
}

#[test]
fn assignment_function() {
    let parsed = ProgramParser::new()
        .parse("let foo = ||: Int => { 5; };")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_void(), ());
}
