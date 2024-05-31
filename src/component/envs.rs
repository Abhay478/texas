use std::fmt::Display;

use crate::prelude::*;

/// OG environment, with \begin{} ... \end{}. For lists, please use `List`.
/// Halfway through implementing arguments, stay tuned.
#[derive(Debug, Clone)]
pub struct Environment {
    name: String,
    components: Vec<Component>,
    opt: Vec<String>,
}
impl AsLatex for Environment {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        let opts = self.opt.join(", ");
        format!(
            "\\begin{{{}}}[{}] \n {} \n \\end{{{}}} \n ",
            self.name, opts, comps, self.name
        )
    }
}
impl Populate for Environment {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.components.extend(other.into_iter());
        Ok(self)
    }
}
impl Opt for Environment {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string());
    }
}
impl Environment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
            opt: vec![],
        }
    }
}

/// OG List, itemize or enumerate. If y'all want description, please put up an issue.
#[derive(Debug, Clone)]
pub struct List {
    items: Vec<Component>,
    typ: ListType,
    opt: Vec<String>,
}
impl AsLatex for List {
    fn to_string(&self) -> String {
        let comps = self
            .items
            .iter()
            .map(|s| format!("\t\\item {}\n", s.to_string()))
            .collect::<String>();
        let opts = self.opt.join(", ");
        format!(
            "\\begin{{{}}}[{}] \n {} \n \\end{{{}}} \n ",
            self.typ.to_string(),
            opts,
            comps,
            self.typ.to_string()
        )
    }
}
impl Populate for List {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.items.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.items.extend(other.into_iter());
        Ok(self)
    }
}
impl Opt for List {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string());
    }
}
impl List {
    pub fn new(typ: ListType) -> Self {
        Self {
            items: vec![],
            typ,
            opt: vec![],
        }
    }

    pub fn with_items(typ: ListType, items: Vec<Component>) -> Self {
        Self {
            items,
            typ,
            opt: vec![],
        }
    }
}

/// Variants for itemize and enumerate.
#[derive(Debug, Clone)]
pub enum ListType {
    Itemize,
    Enumerate,
}
impl Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Itemize => "itemize",
                Self::Enumerate => "enumerate",
            }
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Figure {
    img: Image,
    caption: String,
    opt: Vec<String>,
}

impl AsLatex for Figure {
    fn to_string(&self) -> String {
        format!(
            "\\begin{{figure}}[{}] \n \\centering \n {} \n \\caption{{{}}} \n \\end{{figure}} ",
            self.opt.join(", "),
            self.img.to_string(),
            self.caption
        )
    }
}

impl Opt for Figure {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string());
    }
}
