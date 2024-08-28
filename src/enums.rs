use super::*;

// Define an enum to represent the status of XML validation
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Valid,   // Represents a valid XML string
    Invalid, // Represents an invalid XML string
}

// Implement the Display trait for the Status enum
// This allows the Status to be converted into a string for output
impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Match the enum variant and write the corresponding string
        match self {
            Self::Valid => write!(f, "Valid"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}

// Implement a conversion method for the Status enum
impl Status {
    // Convert a boolean value into a Status enum variant
    // true -> Valid, false -> Invalid
    pub fn from(val: bool) -> Self {
        match val {
            true => Self::Valid,
            false => Self::Invalid,
        }
    }
}
