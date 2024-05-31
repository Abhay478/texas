use std::{
    fs::File,
    io::{self, Read},
};

use crate::prelude::*;

/// Italics and stuff. Also includes the mathy \\(..\\) and \\[...\\], as well as the Scope variant, \\{...\\}
/// If y'all want more (like \textbb{}, \texttt{}, etc.) please put up an issue.
/// A few fonts come from packages, which I'm not handling.
/// You'll get a latex error if you don't also include the package.
#[derive(Debug, Clone)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Teletype,
    MathBold,
    MathCal,
    MathBb,
    MathRm,
    Underlined,
    InlineMath,
    DisplayMath,
    Scope,
    Verbatim,
    Strikethrough,
}

/// Basic text struct. Typically, a `Paragraph` or `Line` contains a bunch of these.
/// Can also read from a file, in which case `typ` will be Normal.
#[derive(Debug, Clone)]
pub struct TextChunk {
    body: String,
    typ: TextType,
}
impl AsLatex for TextChunk {
    fn to_string(&self) -> String {
        match &self.typ {
            TextType::Normal => format!("{} ", self.body),
            TextType::Italic => format!("\\textit{{{}}} ", self.body),
            TextType::Bold => format!("\\textbf{{{}}} ", self.body),
            TextType::Teletype => format!("\\texttt{{{}}} ", self.body),
            TextType::MathBold => format!("\\mathbf{{{}}} ", self.body),
            TextType::MathCal => format!("\\mathcal{{{}}} ", self.body),
            TextType::MathBb => format!("\\mathbb{{{}}} ", self.body),
            TextType::MathRm => format!("\\mathrm{{{}}} ", self.body),
            TextType::Underlined => format!("\\underline{{{}}} ", self.body),
            TextType::InlineMath => format!("\\({}\\)", self.body),
            TextType::DisplayMath => format!("\\[{}\\]", self.body),
            TextType::Scope => format!("\\{{{}\\}}", self.body),
            TextType::Verbatim => format!("\\verb|{}|", self.body),
            TextType::Strikethrough => format!("\\sout{{{}}} ", self.body),
        }
    }
}
impl Populate for TextChunk {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        if let Component::TextChunk(ch) = other {
            self.body.push_str(&ch.body);
            Ok(self)
        } else {
            Err(TexError::RankMismatch.into())
        }
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        for c in other {
            if let Component::TextChunk(ch) = c {
                self.body.push_str(&ch.body);
            } else {
                return Err(TexError::RankMismatch.into());
            }
        }
        Ok(self)
        // todo!()
    }
}
impl TextChunk {
    pub fn new(body: &str, typ: TextType) -> Self {
        Self {
            body: body.to_string(),
            typ,
        }
    }
    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let mut f = File::open(path)?;
        let mut buf = "".to_string();
        f.read_to_string(&mut buf)?;
        Ok(Self {
            body: buf,
            typ: TextType::Normal,
        })
    }
}
