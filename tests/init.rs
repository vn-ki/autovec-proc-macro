#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/compile-test-01.rs");
    t.compile_fail("tests/compile-test-no-args.rs");
    t.compile_fail("tests/compile-test-no-return.rs");
    t.pass("tests/compile-test-tuple.rs");
    t.pass("tests/compile-test-generic.rs");
}
