// commands, macros and stuff.

use crate::prelude::*;

/// Latex macros.
/// Rudimentary so far, have to embed latex.
/// Please refer to README for usage.
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub nargs: usize,
    pub def: String, // actual latex, cannot help
}

impl Command {
    /// You use this to make a new command (duh), but you need the `doc.new_command()` call to actually be able to access it within the document.
    pub fn new(name: &str, nargs: usize, def: &str) -> Self {
        Self {
            name: name.to_string(),
            nargs,
            def: def.to_string(),
        }
    }

    /// I'd really prefer you try and use the `command!` macro.
    pub fn call(&self, args: Vec<&str>) -> TexResult<String> {
        if args.len() != self.nargs {
            return Err(TexError::ArgLen.into());
        }
        let temp = format!(
            "\\{}{}",
            self.name,
            args.iter().map(|x| format!("{{{x}}}")).collect::<String>()
        );
        Ok(temp)
    }

    /// \\newcommand
    pub fn declare(&self) -> String {
        format!(
            "\\newcommand{{\\{}}}[{}]{{{}}} ",
            self.name, self.nargs, self.def
        )
    }
}
