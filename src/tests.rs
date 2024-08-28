use crate::{determine_xml, Status};
use test_case::test_case;

// Unit tests for the determine_xml function
// Each test case checks a different XML string and its expected validity

#[test_case("<Design><Code>hello world</Code></Design>", Status::Valid ; "normal case")]
#[test_case("<Design><Code>hello world</Code></Design><People>", Status::Invalid ; "no closing tag")]
#[test_case("<People><Design><Code>hello world</People></Code></Design>", Status::Invalid ; "non-corresponding tags")]
// there is no closing tag for "People age=”1”" and no opening tag for "/People"
#[test_case("<People age=”1”>hello world</People>", Status::Invalid ; "attribute is not supported")]
#[test_case("<People age=”1”>hello world</People age=”1”>", Status::Valid ; "attribute is supported")]
#[test_case("<Design>Hello World", Status::Invalid ; "incomplete tag")]
#[test_case("Hello World</Design>", Status::Invalid ; "single tag without opening")]
#[test_case("Hello World", Status::Valid ; "without tags")]
#[test_case("", Status::Valid ; "empty xml string")]
#[test_case("<tag></tag>", Status::Valid ; "single pair tags without content")]
#[test_case("<tag1></tag1>Hello World<tag2></tag2>", Status::Valid ; "multiple tags at same level")]
#[test_case("<tag />", Status::Invalid ; "self closing tag")]
#[test_case("<tag attribute=\"value\"></tag>", Status::Invalid ; "self cloing tag")]
#[test_case("<tag1><tag2><tag3></tag3><tag4></tag4></tag2></tag1>", Status::Valid ; "multiple tags")]
#[test_case("<tag1><tag2><tag3></tag4><tag4></tag3></tag2></tag1>", Status::Invalid ; "multiple tags with incorrect order")]
fn check_determine_xml(input: &'static str, expected: Status) {
    let input = input.to_string();
    assert_eq!(expected, determine_xml(&input));
}
