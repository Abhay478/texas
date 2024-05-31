use crate::prelude::*;

/// Images!
/// Please enable images for the current document before, using: `doc.enable_graphicx(path)`
#[derive(Debug, Clone)]
pub struct Image {
    path: String,
    opt: Vec<String>,
}
impl AsLatex for Image {
    fn to_string(&self) -> String {
        let options = self
            .opt
            .iter()
            .map(|s| format!("{}, ", s))
            .collect::<String>();
        format!("\\includegraphics[{}]{{{}}} \n", options, self.path)
    }
}
impl Opt for Image {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string());
    }
}
impl Image {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            opt: vec![],
        }
    }

    pub fn with_options(path: &str, opt: Vec<String>) -> Self {
        Self {
            path: path.to_string(),
            opt,
        }
    }
}
