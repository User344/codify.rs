// This is free and unencumbered software released into the public domain.

/// See: https://en.cppreference.com/w/cpp/language/types
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Type {
    /// See: https://en.cppreference.com/w/cpp/keyword/bool
    Bool,
}

impl core::str::FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "bool" => Self::Bool,
            _ => return Err(()),
        })
    }
}
