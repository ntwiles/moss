use moss::{
    grammar::ProgramParser,
    test_util::{analyze_program, exec_program},
};

#[test]
fn search_parent_scope() {
    let code = r#"
        let foo = 2 + 5;
        let bar = ||: Int => { foo; };
        bar();
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    let analyzed = analyze_program(parsed).unwrap();
    let result = exec_program(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

#[test]
fn search_parent_scope_non_closure_type_error() {
    let code = r#"
        let foo = 2 + 5;
        let bar = (): Int => { foo; };
        bar();
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    analyze_program(parsed).expect_err("() => {} syntax should not create a closure.");
}

#[test]
fn search_child_scope_error_type_error() {
    let code = r#"
        let foo = (): Void => { let bar = 2 + 5; };
        foo();
        bar;
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    analyze_program(parsed).expect_err("bar is not in scope.");
}
