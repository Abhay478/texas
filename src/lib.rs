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
//! Mine's still better, though. 😉
//! 



use std::error::Error;

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

/// 
#[cfg(test)]
mod tests;

pub use traits::*;
pub use commands::*;
pub use component::*;
pub use document::*;
pub use errors::*;

type Res<T> = Result<T, Box<dyn Error>>;
type Null = Res<()>;
