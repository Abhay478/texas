use crate::prelude::*;
use std::fmt::Display;

/// Currently, only these few types are supported.
/// There is also nothing preventing you from putting a \part{} in a document of class "part",
/// but latex will show an error. If you want those restrictions to be implemented, please put
/// up an issue
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentClassType {
    Article,
    Amsart,
    Part,
    Report,
    Book,
    Beamer,
}
impl Display for DocumentClassType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Article => "article",
                Self::Part => "part",
                Self::Report => "report",
                Self::Book => "book",
                Self::Amsart => "amsart",
                Self::Beamer => "beamer",
            }
        )?;

        Ok(())
    }
}
impl From<&str> for DocumentClassType {
    fn from(value: &str) -> Self {
        match value {
            "article" => Self::Article,
            "part" => Self::Part,
            "book" => Self::Book,
            "report" => Self::Report,
            "amsart" => Self::Amsart,
            "beamer" => Self::Beamer,
            _ => Self::Article,
        }
    }
}

/// Wrapper around `DocumentClassType`, contains whatever options you want to add.
/// Nothing prevents you from adding absolute gibberish as an option. If you want those
/// restrictions implemented, please put up an issue.
#[derive(Debug, Clone)]
pub struct DocumentClass {
    pub(crate) typ: DocumentClassType,
    pub(crate) opt: Vec<String>,
}
impl DocumentClass {
    pub fn new(typ: &str) -> Self {
        Self {
            typ: typ.into(),
            opt: vec![],
        }
    }
}
impl AsLatex for DocumentClass {
    fn to_string(&self) -> String {
        let options = self
            .opt
            .iter()
            .map(|s| format!("{}, ", s))
            .collect::<String>();
        format!("\\documentclass[{}]{{{}}}", options, self.typ.to_string())
    }
}
impl Opt for DocumentClass {
    fn add_option(&mut self, opt: &str) {
        self.opt.push(opt.to_string())
    }
}
