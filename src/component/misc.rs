use crate::prelude::*;

/// \input{}, if you want that kinda thing. ~Personally, I've never used it.~
/// I _have_ used it. Pretty useful.
#[derive(Debug, Clone)]
pub struct Input {
    name: String,
}
impl AsLatex for Input {
    fn to_string(&self) -> String {
        format!("\\input{{{}}}", self.name)
    }
}
