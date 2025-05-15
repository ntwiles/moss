use moss::{grammar::ProgramParser, test_util};

#[test]
fn search_parent_scope() {
    let code = r#"
        let foo = 2 + 5;
        let bar = ||: Int => { foo; };
        bar();
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    let analyzed = test_util::analyze_program(parsed).unwrap();
    let result = test_util::exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}
