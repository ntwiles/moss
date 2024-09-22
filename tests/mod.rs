use lang_2::analyzer;
use lang_2::grammar::ProgramParser;
use lang_2::interpretor;

#[test]
fn operation_precedence() {
    let parsed = ProgramParser::new().parse("10 + 5 * 2 - 8 / 4").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_int(), 18);
}

#[test]
fn operation_precedence_with_negatives() {
    let parsed = ProgramParser::new().parse("-10 + -5 * 2 - -8 / 4").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_int(), -18);
}

#[test]
fn equality_comparison_true() {
    let parsed = ProgramParser::new().parse("15 - 5 == 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn equality_comparison_false() {
    let parsed = ProgramParser::new().parse("15 + 5 == 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn greater_than_comparison_true() {
    let parsed = ProgramParser::new().parse("15 + 5 > 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn greater_than_comparison_false() {
    let parsed = ProgramParser::new().parse("15 - 5 > 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn less_than_comparison_true() {
    let parsed = ProgramParser::new().parse("10 - 5 < 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn less_than_comparison_false() {
    let parsed = ProgramParser::new().parse("15 + 5 < 5 + 5").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), false);
}

#[test]
fn boolean_literal_true() {
    let parsed = ProgramParser::new().parse("true == true").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), true);
}

#[test]
fn boolean_literal_false() {
    let parsed = ProgramParser::new().parse("true == false").unwrap();

    let analyzed = analyzer::analyze_expr(&parsed).unwrap();
    let result = interpretor::interpret_expr(analyzed);

    assert_eq!(result.unwrap_bool(), false);
}
