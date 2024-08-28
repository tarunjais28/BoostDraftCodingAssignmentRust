# Pull Request Description

### Title: **Add XML Validation Logic with Status Enum and Unit Tests**

---

## Summary
This pull request introduces an XML validation feature to the project. The code validates whether an input XML string is properly structured, ensuring that all tags are correctly opened and closed in a well-nested order. The validation status (`Valid` or `Invalid`) is returned as a custom `Status` enum, which is then printed to the console.

## Changes Made
1. **Main Functionality**:
   - **`determine_xml` Function**: 
     - Implements XML validation logic using a stack to ensure tags are correctly nested and matched.
     - Returns a `Status` enum (`Valid` or `Invalid`) based on the validity of the XML.
   - **`Status` Enum**:
     - Defines two variants: `Valid` and `Invalid`.
     - Implements `Display` trait to allow easy conversion to string for output.

2. **Testing**:
   - Added unit tests for the `determine_xml` function.
   - Used the `test_case` crate to define multiple test cases, covering a wide range of XML scenarios including normal cases, edge cases, and incorrect tag orders.

## Usage Instructions

### Runing the Release Version:
To run the release version of the application, use the following command:

```bash
cargo run --release
```
This will compile the application with optimizations enabled, producing an optimized binary in the target/release directory.

### Running the Application
To run the application and validate an XML string, use the following command:

```bash
cargo run -- "<your_xml_string_here>"
```

- Replace `"<your_xml_string_here>"` with your actual XML string.
- Example:

```bash
cargo run -- "<Design><Code>hello world</Code></Design>"
```

### Running the Tests
To ensure the validity of the XML validation logic, run the unit tests using:

```bash
cargo test
```

This command will execute all the test cases defined in the `tests` module, checking for both valid and invalid XML strings.

---

## Example Output

- **Valid XML Example**:

  ```bash
  $ cargo run -- "<Design><Code>hello world</Code></Design>"
  Valid
  ```

- **Invalid XML Example**:

  ```bash
  $ cargo run -- "<Design><Code>hello world</Code></Design><People>"
  Invalid
  ```

## Test Coverage
The test cases cover:
- Properly nested tags.
- Incorrectly ordered closing tags.
- Single tags without matching pairs.
- Edge cases like empty strings, strings without tags, and self-closing tags (which are considered invalid in this simplified scenario).

## Additional Notes
- The `Status` enum provides a clear and extensible way to handle and output the validation results.
- The XML validation logic currently does not support self-closing tags or attributes within tags as valid, per the simplified requirements. Further enhancements could be made to handle these cases if needed.

---

This pull request ensures that the XML validation logic is both robust and well-tested, providing accurate results for a wide range of input scenarios.
