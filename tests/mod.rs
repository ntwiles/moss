use moss::analyzer;
use moss::grammar::ProgramParser;
use moss::interpretor;
use moss::shared::builtins::get_builtins;

#[test]
fn operation_precedence() {
    let parsed = ProgramParser::new().parse("10 + 5 * 2 - 8 / 4;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_int(), 18);
}

#[test]
fn operation_precedence_with_negatives() {
    let parsed = ProgramParser::new()
        .parse("-10 + -5 * 2 - -8 / 4;")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_int(), -18);
}

#[test]
fn equality_comparison_true() {
    let parsed = ProgramParser::new().parse("15 - 5 == 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn equality_comparison_false() {
    let parsed = ProgramParser::new().parse("15 + 5 == 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn greater_than_comparison_true() {
    let parsed = ProgramParser::new().parse("15 + 5 > 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn greater_than_comparison_false() {
    let parsed = ProgramParser::new().parse("15 - 5 > 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn less_than_comparison_true() {
    let parsed = ProgramParser::new().parse("10 - 5 < 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn less_than_comparison_false() {
    let parsed = ProgramParser::new().parse("15 + 5 < 5 + 5;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn boolean_literal_true() {
    let parsed = ProgramParser::new().parse("true == true;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn boolean_literal_false() {
    let parsed = ProgramParser::new().parse("true == false;").unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn string_concatenation() {
    let parsed = ProgramParser::new()
        .parse("\"hello\" + \" world\";")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_string(), "hello world");
}

#[test]
fn string_concatenation_assigned() {
    let parsed = ProgramParser::new()
        .parse("let foo = \"hello\"; foo + \" world\";")
        .unwrap();

    let analyzed = analyzer::analyze_program(parsed, get_builtins()).unwrap();
    let result = interpretor::interpret_program(analyzed, get_builtins()).unwrap();

    assert_eq!(result.unwrap_string(), "hello world");
}
