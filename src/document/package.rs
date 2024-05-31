use crate::prelude::*;

/// This here is the main reason I made this crate - other crates don't let you add options
/// to packages. Has macro support, so please use it :).
#[derive(Debug, Clone)]
pub struct Package {
    pub(crate) name: String,
    pub(crate) opt: Vec<String>,
}
impl AsLatex for Package {
    fn to_string(&self) -> String {
        let options = self
            .opt
            .iter()
            .map(|s| format!("{}, ", s))
            .collect::<String>();
        format!("\\usepackage[{}]{{{}}}\n", options, self.name)
    }
}
impl Opt for Package {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string())
    }
}
impl Package {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            opt: vec![],
        }
    }
}
