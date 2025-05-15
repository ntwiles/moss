use moss::{grammar::ProgramParser, test_util};

#[test]
fn search_parent_scope_non_closure_type_error() {
    let code = r#"
        let foo = 2 + 5;
        let bar = (): Int => { foo; };
        bar();
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    test_util::analyze_program(parsed).expect_err("() => {} syntax should not create a closure.");
}

#[test]
fn search_child_scope_error_type_error() {
    let code = r#"
        let foo = (): Void => { let bar = 2 + 5; };
        foo();
        bar;
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    test_util::analyze_program(parsed).expect_err("bar is not in scope.");
}
