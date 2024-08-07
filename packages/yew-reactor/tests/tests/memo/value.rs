use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub(super) enum Value {
    None,
    Int(i32),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::None => write!(f, "none"),
            Self::Int(i) => write!(f, "int:{i}"),
            Self::String(s) => write!(f, "str:{s}"),
        }
    }
}
