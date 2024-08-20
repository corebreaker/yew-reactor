use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Default)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Male => write!(f, "M"),
            Self::Female => write!(f, "F"),
        }
    }
}

impl Debug for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
        }
    }
}

impl PartialEq for Gender {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Male, Self::Male)|(Self::Female, Self::Female) => true,
            _ => false,
        }
    }
}

impl Eq for Gender {}
