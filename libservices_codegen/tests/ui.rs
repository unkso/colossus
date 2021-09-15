use trybuild::TestCases;

#[test]
fn test_ui() {
    let tests = TestCases::new();
    tests.pass("tests/pass/generic.rs");
    tests.compile_fail("tests/fail/missing_attr.rs");
    tests.compile_fail("tests/fail/invalid_ident.rs");
}
