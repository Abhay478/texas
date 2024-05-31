use crate::prelude::*;
pub trait AsLatex {
    fn to_string(&self) -> String;
}

pub trait Populate {
    /// Weird return type for chaining attaches.
    fn attach(&mut self, other: Component) -> TexResult<&mut Self>;
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self>;
}

pub trait Opt {
    fn add_option(&mut self, opt: &str);
}
