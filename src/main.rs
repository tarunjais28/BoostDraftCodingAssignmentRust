use std::env;
use std::collections::VecDeque;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let default_input = String::default();
    let input = args.first().unwrap_or(&default_input);

    let result = if determine_xml(input) {
        "Valid"
    } else {
        "Invalid"
    };
    println!("{}", result);
}

fn determine_xml(input: &str) -> bool {
    let mut stack: VecDeque<String> = VecDeque::new();
    let mut i = 0;

    while i < input.len() {
        if &input[i..i + 1] == "<" {
            // Find the closing '>'
            if let Some(j) = input[i..].find('>') {
                let tag = &input[i + 1..i + j];
                if tag.starts_with("/") {
                    // Closing tag
                    let tag_name = &tag[1..];
                    if stack.pop_back() != Some(tag_name.to_string()) {
                        return false;
                    }
                } else {
                    // Opening tag
                    stack.push_back(tag.to_string());
                }
                i += j + 1;
            } else {
                return false;
            }
        } else {
            i += 1;
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::determine_xml;

    // You can use here to test, feel free to modify/add the test cases here.
    // You can run tests with `cargo test`.
    // You can also use other ways to test if you want.

    #[test_case("<Design><Code>hello world</Code></Design>", true ; "normal case")]
    #[test_case("<Design><Code>hello world</Code></Design><People>", false ; "no closing tag")]
    #[test_case("<People><Design><Code>hello world</People></Code></Design>", false ; "non-corresponding tags")]
    // there is no closing tag for "People age=”1”" and no opening tag for "/People"
    #[test_case("<People age=”1”>hello world</People>", false ; "attribute is not supported")]
    #[test_case("<People age=”1”>hello world</People age=”1”>", true ; "attribute is supported")]
    #[test_case("<Design>Hello World", false ; "incomplete tag")]
    #[test_case("Hello World</Design>", false ; "single tag without opening")]
    #[test_case("Hello World", true ; "without tags")]
    #[test_case("", true ; "empty xml string")]
    #[test_case("<tag></tag>", true ; "single pair tags without content")]
    #[test_case("<tag1></tag1>Hello World<tag2></tag2>", true ; "multiple tags at same level")]
    #[test_case("<tag />", false ; "self closing tag")]
    #[test_case("<tag attribute=\"value\"></tag>", false ; "self cloing tag")]
    #[test_case("<tag1><tag2><tag3></tag3><tag4></tag4></tag2></tag1>", true ; "multiple tags")]
    #[test_case("<tag1><tag2><tag3></tag4><tag4></tag3></tag2></tag1>", false ; "multiple tags with incorrect order")]
    fn check_determine_xml(input: &'static str, expected: bool) {
        let input = input.to_string();
        assert_eq!(expected, determine_xml(&input));
    }
}
