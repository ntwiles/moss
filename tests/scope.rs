use moss::{analyzer, grammar::ProgramParser, interpretor};

#[test]
fn search_parent_scope() {
    let code = r#"
        let foo = 2 + 5;
        let bar = || => { foo; };
        bar();
    "#;

    let parsed = ProgramParser::new().parse(code).unwrap();

    let analyzed = analyzer::analyze_program(parsed).unwrap();
    let result = interpretor::interpret_lines(analyzed).unwrap();

    assert_eq!(result.unwrap_int(), 7);
}

// TODO: Handle this once these cause runtime errors.
// #[test]
// fn search_parent_scope_non_closure_type_error() {
//     let code = r#"
//         let foo = 2 + 5;
//         let bar = () => { foo; };
//         bar();
//     "#;

//     let parsed = ProgramParser::new().parse(code).unwrap();

//     analyzer::analyze_program(parsed).expect_err(
//         "TODO: This should be a TypeError as the () => {} syntax should not create a closure.",
//     );
// }

// #[test]
// fn search_child_scope_error_type_error() {
//     let code = r#"
//         let foo = () => { let bar = 2 + 5; };
//         foo();
//         bar;
//     "#;

//     let parsed = ProgramParser::new().parse(code).unwrap();

//     analyzer::analyze_program(parsed)
//         .expect_err("TODO: This should be a TypeError as bar is not in scope.");
// }
