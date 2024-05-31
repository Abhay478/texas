use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Block {
    title: String,
    components: Vec<Component>,
}

impl AsLatex for Block {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!(
            "\\begin{{block}}{{{}}} \n {} \\end{{block}} \n ",
            self.title, comps
        )
    }
}

impl Populate for Block {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> TexResult<&mut Self> {
        self.components.append(&mut other);
        Ok(self)
    }
}

impl Block {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            components: vec![],
        }
    }

    pub fn new_untitled() -> Self {
        Self::new("")
    }

    pub fn untitled_with_components(components: Vec<Component>) -> Self {
        Self::with_components("", components)
    }

    pub fn with_components(title: &str, components: Vec<Component>) -> Self {
        Self {
            title: title.to_string(),
            components,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    title: String,
    components: Vec<Component>,
}

impl AsLatex for Frame {
    fn to_string(&self) -> String {
        let comps = self
            .components
            .iter()
            .map(|s| s.to_string())
            .collect::<String>();
        format!(
            "\\begin{{frame}}{{{}}} \n {} \\end{{frame}} \n ",
            self.title, comps
        )
    }
}

impl Populate for Frame {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> TexResult<&mut Self> {
        self.components.append(&mut other);
        Ok(self)
    }
}

impl Frame {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            components: vec![],
        }
    }

    pub fn new_untitled() -> Self {
        Self::new("")
    }

    pub fn untitled_with_components(components: Vec<Component>) -> Self {
        Self::with_components("", components)
    }

    pub fn with_components(title: &str, components: Vec<Component>) -> Self {
        Self {
            title: title.to_string(),
            components,
        }
    }
}
