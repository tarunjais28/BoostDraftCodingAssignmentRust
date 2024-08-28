use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let default_input = "".to_string();
    let input = args.get(0).unwrap_or(&default_input);

    let result = if determine_xml(input) {
        "Valid"
    } else {
        "Invalid"
    };
    println!("{}", result);
}

// feel free to add other classes/methods if you want
fn determine_xml(input: &String) -> bool {
    // TODO: Implement here
    return false;
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
    fn check_determine_xml(input: &'static str, expected: bool) {
        let input = input.to_string();
        assert_eq!(expected, determine_xml(&input));
    }
}
