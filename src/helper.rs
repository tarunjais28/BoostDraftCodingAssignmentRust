use super::*;

// Function to determine xml tags are valid or not
pub fn determine_xml(input: &str) -> Status {
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
