use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Builtin {
    typ: BuiltinType,
}
impl AsLatex for Builtin {
    fn to_string(&self) -> String {
        format!("{}", self.typ.to_string())
    }
}
impl Builtin {
    pub fn new(typ: BuiltinType) -> Self {
        Self { typ }
    }
}
#[derive(Debug, Clone)]
pub enum BuiltinType {
    EnsureMath(TextChunk),
    Sin(TextChunk),
    Cos(TextChunk),
    Tan(TextChunk),
    Log(TextChunk),
    Ln(TextChunk),
    Lg(TextChunk),
    Sum(TextChunk, TextChunk),
    Prod(TextChunk, TextChunk),
    Arg(TextChunk),
    Min(TextChunk),
    Max(TextChunk),
    Character(String), // As in greek, but also stuff like \infty, etc.
}
impl Display for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnsureMath(s) => format!("\\ensuremath{{{}}}", s.to_string()),
                Self::Sin(s) => format!("\\sin{{{}}}", s.to_string()),
                Self::Cos(s) => format!("\\cos{{{}}}", s.to_string()),
                Self::Tan(s) => format!("\\tan{{{}}}", s.to_string()),
                Self::Log(s) => format!("\\log{{{}}}", s.to_string()),
                Self::Ln(s) => format!("\\ln{{{}}}", s.to_string()),
                Self::Lg(s) => format!("\\lg{{{}}}", s.to_string()),
                Self::Sum(down, up) =>
                    format!("\\sum_{{{}}}^{{{}}}", down.to_string(), up.to_string()),
                Self::Prod(down, up) =>
                    format!("\\prod_{{{}}}^{{{}}}", down.to_string(), up.to_string()),
                Self::Arg(s) => format!("\\arg{{{}}}", s.to_string()),
                Self::Min(s) => format!("\\min{{{}}}", s.to_string()),
                Self::Max(s) => format!("\\max{{{}}}", s.to_string()),
                Self::Character(s) => format!("\\{}", s),
            }
        )?;

        Ok(())
    }
}
