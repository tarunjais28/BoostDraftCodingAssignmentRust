use std::collections::VecDeque;
use std::env;
use std::fmt::Display;

fn main() {
    // Collect command line arguments into a vector of strings
    let args = env::args().collect::<Vec<_>>();

    // Provide a default empty string in case no arguments are provided
    let default_input = String::default();

    // Get the first argument if available, otherwise use the default empty string
    let input = args.first().unwrap_or(&default_input);

    // Determine and print if the input XML string is valid or not
    println!("{}", determine_xml(input));
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Valid,
    Invalid,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid => write!(f, "Valid"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}

impl Status {
    fn from(val: bool) -> Self {
        match val {
            true => Self::Valid,
            false => Self::Invalid,
        }
    }
}

// Function to determine xml tags are valid or not
fn determine_xml(input: &str) -> Status {
    // Initialize a stack to keep track of open tags
    let mut stack: VecDeque<String> = VecDeque::new();
    let mut i = 0; // Pointer to iterate through the input string

    // Loop through each character in the input string
    while i < input.len() {
        // Check if the current character is the start of a tag
        if &input[i..i + 1] == "<" {
            // Find the closing '>' of the tag
            if let Some(j) = input[i..].find('>') {
                let tag = &input[i + 1..i + j]; // Extract the tag name

                // Check if it's a closing tag (starts with '/')
                if let Some(stripped_tag) = tag.strip_prefix("/") {
                    // Pop the last open tag from the stack and compare it with the closing tag
                    if stack.pop_back() != Some(stripped_tag.to_string()) {
                        return Status::Invalid; // If they don't match, the XML is invalid
                    }
                } else {
                    // If it's an opening tag, push it onto the stack
                    stack.push_back(tag.to_string());
                }

                // Move the pointer past the current tag
                i += j + 1;
            } else {
                return Status::Invalid; // If no closing '>' is found, the XML is invalid
            }
        } else {
            i += 1; // Move to the next character if it's not a tag
        }
    }

    // The XML is valid only if the stack is empty at the end (all tags are closed properly)
    Status::from(stack.is_empty())
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::{determine_xml, Status};

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
}
