use crate::prelude::*;

/// \part{}: Only available for \documentclass{book}
#[derive(Debug, Clone)]
pub struct Part {
    name: String,
    components: Vec<Component>,
}
impl AsLatex for Part {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!("\\part{{{}}} \n {} \n ", self.name, comps)
    }
}
impl Populate for Part {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Part {
    pub fn new(name: &str) -> Self {
        Self {
            name: escape(name, None),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: escape(name, None),
            components,
        }
    }
}

/// \chapter{}: Only available for \documentclass{book}
#[derive(Debug, Clone)]
pub struct Chapter {
    name: String,
    components: Vec<Component>,
}
impl AsLatex for Chapter {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!("\\chapter{{{}}} \n {} \n ", self.name, comps)
    }
}
impl Populate for Chapter {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Chapter {
    pub fn new(name: &str) -> Self {
        Self {
            name: escape(name, None),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: escape(name, None),
            components,
        }
    }
}

/// \section{}: Major partitioning device within a document
#[derive(Debug, Clone)]
pub struct Section {
    name: String,
    components: Vec<Component>,
}
impl AsLatex for Section {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!("\\section{{{}}} \n {} \n ", self.name, comps)
    }
}
impl Populate for Section {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Section {
    pub fn new(name: &str) -> Self {
        // let name = name.to_string().replace("_", "\\_")

        Self {
            name: escape(name, None),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: escape(name, None),
            components,
        }
    }
}

/// \subsection{}
#[derive(Debug, Clone)]
pub struct Subsection {
    name: String,
    components: Vec<Component>,
}
impl AsLatex for Subsection {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!("\\subsection{{{}}} \n {} \n ", self.name, comps)
    }
}
impl Populate for Subsection {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Subsection {
    pub fn new(name: &str) -> Self {
        Self {
            name: escape(name, None),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: escape(name, None),
            components,
        }
    }
}

/// Block of text bracketed by "\n\n". Generates a latex paragraph.
#[derive(Debug, Clone)]
pub struct Paragraph {
    components: Vec<Component>,
}
impl AsLatex for Paragraph {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!("\n\n {} \n\n ", comps)
    }
}
impl Populate for Paragraph {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Paragraph {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn with_components(components: Vec<Component>) -> Self {
        Self { components }
    }
}

/// Terminated by "\\ \n", causes linebreaks within the document.
#[derive(Debug, Clone)]
pub struct Line {
    components: Vec<Component>,
}
impl AsLatex for Line {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        if comps.trim().is_empty() {
            format!("\n")
        } else {
            format!("{} \\\\\n", comps)
        }
    }
}
impl Populate for Line {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        self.attach_iter(other.into_iter())
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, other: I) -> TexResult<&mut Self> {
        self.components.extend(other);
        Ok(self)
    }
}
impl Line {
    pub fn new() -> Self {
        Self { components: vec![] }
    }

    pub fn with_components(components: Vec<Component>) -> Self {
        Self { components }
    }
}
