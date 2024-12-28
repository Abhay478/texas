use std::{error::Error, fmt::Display, io};

/// Your garden-variety custom error.
/// Contains the catch-all `WhatEven` variant (WhatEven as in "What even is this?")
/// If y'all want more, please put up an issue.
#[derive(Debug)]
pub enum TexError {
    ArgLen,
    RankMismatch(u8, u8),
    WhatEven(String),
    TraitUnimplemented(String),
    VariantUndefined,
    LabelUndefined,
    Undefined,
    // #[cfg(feature = "markdown")]
    // MarkdownError(String),
    IoError(io::Error),
}

impl Display for TexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                TexError::ArgLen => "Incorrect number of arguments.".to_string(),
                TexError::RankMismatch(a,b) => format!("Rank mismatch: {a} < {b}."),
                TexError::WhatEven(s) => s.to_string(),
                TexError::Undefined => "Object not defined.".to_string(),
                TexError::VariantUndefined => "The literal you provided does not correspond to a Variant. Please refer to the documentation for the list of valid literals.".to_string(),
                TexError::LabelUndefined => "The label you provided does not exist.".to_string(),
                TexError::TraitUnimplemented(s) => format!("{} does not implement the trait you desire (probably Populate).", s.to_string()),
                // #[cfg(feature = "markdown")]
                // TexError::MarkdownError(message) => message.to_string(),
                TexError::IoError(e) => e.to_string()
            }
        )?;

        Ok(())
    }
}

impl Error for TexError {}

impl From<io::Error> for TexError {
    fn from(value: io::Error) -> Self {
        TexError::IoError(value)
    }
}

pub type TexResult<T> = Result<T, TexError>;
