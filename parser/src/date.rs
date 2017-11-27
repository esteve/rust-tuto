use regex;

#[derive(Clone, Debug, PartialEq)]
pub struct Date {
    // TODO
}

/// This is a doc comment
pub fn parse_date(src: &str) -> Date {
    
    unimplemented!()
}

// This is a test
// run it with the command: "cargo test"
#[test]
fn parse_a_date() {
    // this test will fail because parse_date is unimplemented
    assert_eq!(parse_date(&"20-05-1989"), Date {});
}
