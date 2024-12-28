//!
//! Well, hello there.
//!
//! So this crate aims to provide convenient ways of programmatically generating a latex file.
//!
//! If you want to just convert from some other file format WITHOUT any processing, I recommend `pandoc`.
//!
//! I'll slowly and steadily begin adding more features, and my goal is to be able to cover most standard use-cases of Latex.
//! If you want any feature, raise an issue! I will see it, and while I may not reply (scatterbrained, much?) I will fix it.
//!
//! A shout-out to another crate, `tex-rs`. A few of the design choices I made are based on this crate.

/// Bunch of From<>s, they feel like they might be useful
pub mod casting;

/// Latex commands/macros. Haven't found this in any other crate.
pub mod commands;

/// Standard Latex things.
pub mod component;

/// Packages and the overall latex layout.
pub mod document;

/// Custom error type.
pub mod errors;

/// Really helpful stuff.
pub mod macros;

/// Ubiquitous.
pub mod traits;

pub mod label;

// #[cfg(feature = "markdown")]
// pub mod markdown;

///
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use crate::commands::*;
    pub use crate::component::*;
    pub use crate::document::*;
    pub use crate::errors::*;
    pub use crate::label::*;
    pub use crate::traits::*;

    pub fn escape(s: &str, esc: Option<&[char]>) -> String {
        if esc.is_none() {
            s.replace("_", "\\_")
                .replace("^", "\\^")
                .replace("#", "\\#")
                .replace("&", "\\&")
                .replace("%", "\\%")
                .replace("$", "\\$")
                .replace("{", "\\{")
                .replace("}", "\\}")
        } else {
            let esc = esc.unwrap();
            let mut s = s.to_string();
            for c in esc {
                s = s.replace(&c.to_string(), &format!("\\{}", c));
            }
            s
        }
    }

    // All the macros, again.
    pub use crate::{
        builtin, chapter, command, document, environment, figure, frame, image, label, package,
        part, reference, row, section, tabular, textchunk, unwrap,
    };
}
