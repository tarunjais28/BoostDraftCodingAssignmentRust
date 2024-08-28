use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
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
    pub fn from(val: bool) -> Self {
        match val {
            true => Self::Valid,
            false => Self::Invalid,
        }
    }
}
