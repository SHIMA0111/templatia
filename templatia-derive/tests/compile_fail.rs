// Compile-fail tests using trybuild, per AGENTS.md Testing Policy.

#[test]
fn compile_errors() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/*.rs");
}
