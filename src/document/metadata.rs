use itertools::Itertools;

use crate::prelude::*;

/// Title and author, if y'all want more, please put up an issue.
#[derive(Debug, Clone)]
pub struct Metadata {
    pub(crate) class: DocumentClass,
    pub(crate) title: String,
    pub(crate) author: Vec<String>,
    pub maketitle: bool,
    pub tableofcontents: bool,
    pub date: bool,
}
impl AsLatex for Metadata {
    fn to_string(&self) -> String {
        let title_author = format!(
            "\\title{{{}}}\n\\author{{{}}}\n",
            self.title,
            self.author.iter().join(r"\\ \and ")
        );
        match self.class {
            DocumentClass {
                typ: DocumentClassType::Beamer,
                ..
            } => {
                // todo!()
                format!(
                    "{title_author}\n{}\n",
                    if self.date { r"\date{\today}" } else { "" },
                )
            }
            _ => {
                format!(
                    "{title_author}\n{}\n{}\n{}\n",
                    if self.date { r"\today" } else { "" },
                    if self.maketitle { r"\maketitle" } else { "" },
                    if self.tableofcontents {
                        r"\tableofcontents"
                    } else {
                        ""
                    },
                )
            }
        }
    }
}
impl Metadata {
    pub fn new(class: DocumentClass, title: &str, author: &[&str]) -> Self {
        Self {
            class,
            title: title.to_string(),
            author: author.iter().map(|x| x.to_string()).collect(),
            maketitle: true,
            tableofcontents: false,
            date: false,
        }
    }
}
