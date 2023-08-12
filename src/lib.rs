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
