use moss::{grammar::ProgramParser, test_util};

#[test]
fn call_wrong_arg() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int): Int => { x; }; foo(false);")
        .unwrap();

    test_util::analyze_program(parsed).expect_err("foo expects int argument, got bool.");
}

#[test]
fn call_too_few_args() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int, y: Int): Int => { x + y; }; foo(7);")
        .unwrap();

    test_util::analyze_program(parsed).expect_err("foo expects 2 arguments, got 1.");
}

#[test]
fn call_too_many_args() {
    let parsed = ProgramParser::new()
        .parse("let foo = (x: Int): Int => { x; }; foo(7, 5);")
        .unwrap();

    test_util::analyze_program(parsed).expect_err("foo expects 1 arguments, got 2.");
}

#[test]
fn call_with_wrong_return_type() {
    let code = r"
        let add = (a: Int, b: Int): String => {
        a + b;
    };
    ";

    let parsed = ProgramParser::new().parse(code).unwrap();
    test_util::analyze_program(parsed).expect_err("wrong return type for signature.");
}
