use std::{error::Error, fmt::Display};

/// Your garden-variety custom error.
/// Contains the catch-all `WhatEven` variant (WhatEven as in "What even is this?")
/// If y'all want more, please put up an issue.
#[derive(Debug)]
pub enum TexError {
    ArgLen,
    RankMismatch,
    WhatEven(String),
    TraitUnimplemented,
    Undefined,
}

impl Display for TexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::ArgLen => "Incorrect number of arguments.",
                Self::RankMismatch => "Rank mismatch.",
                Self::WhatEven(s) => s.as_str(),
                Self::Undefined => "Object not defined.",
                Self::TraitUnimplemented => "This variant of component does not implement the trait you desire (probably Populate)."
            }
        )?;

        Ok(())
    }
}

impl Error for TexError {}
