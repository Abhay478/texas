use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

use crate::*;

/// Currently, only these four types are supported.
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
    typ: DocumentClassType,
    opt: Vec<String>,
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

/// This here is the main reason I made this crate - other crates don't let you add options
/// to packages. Has macro support, to please use it :).
#[derive(Debug, Clone)]
pub struct Package {
    name: String,
    opt: Vec<String>,
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

/// Title and author, if y'all want more, please put up an issue.
#[derive(Debug, Clone)]
pub struct Metadata {
    class: DocumentClass,
    title: String,
    author: Vec<String>,
    pub maketitle: bool,
    pub tableofcontents: bool,
    pub date: bool,
}
impl AsLatex for Metadata {
    fn to_string(&self) -> String {
        let title_author = format!(
            "\\title{{{}}}\n\\author{{{}}}\n",
            self.title,
            self.author
                .iter()
                .join(r"\\ \and ")
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
            date: true,
        }
    }
}

/// The king of the land. The `Document` type is where you start.
/// Has macro support.
/// Also contains a restriction on the latex commands you use - you can't use one without
/// declaring it. Atypical of this crate, this particular feature prevents a latex error.
#[derive(Debug, Clone)]
pub struct Document {
    // document_class: DocumentClass,
    packages: Vec<Package>,
    pub metadata: Metadata,
    components: Vec<Component>,
    commands: HashMap<String, Command>,
    img: bool,
    scratch: bool,
    graphics_path: Option<Vec<String>>,
}
impl AsLatex for Document {
    fn to_string(&self) -> String {
        let dc = self.metadata.class.to_string();
        let pkgs = self
            .packages
            .iter()
            .map(|x| x.to_string())
            .collect::<String>();
        let md = if !self.scratch {
            self.metadata.to_string()
        } else {
            "".to_string()
        };
        let beamer_init_frames: String = if self.metadata.class.typ == DocumentClassType::Beamer {
            // Warning: Unused result. Again, cannot n-choose-2 Component Variants.
            let title_frame = Frame::with_components("", vec![textchunk!(r"\titlepage", "normal")]);

            let mut toc = "".to_string();
            if self.metadata.tableofcontents {
                toc = Frame::with_components("", vec![textchunk!(r"\tableofcontents", "normal")])
                    .to_string();
            } ;

            format!("{}\n{}\n", title_frame.to_string(), toc)
        } else {
            "\n".to_string()
        };
        let body = beamer_init_frames
            + &self
                .components
                .iter()
                .map(|x| x.to_string())
                .collect::<String>();

        let cmd = self
            .commands
            .iter()
            .map(|x| format!("{} \n", x.1.declare()))
            .collect::<String>();

        let gpath = if let Some(path) = &self.graphics_path {
            format!(
                "\\graphicspath{{{}}} \n",
                path.iter()
                    .map(|x| format!("{{{}}}, ", x))
                    .collect::<String>()
            )
        } else {
            "".to_string()
        };
        format!(
            "{}\n{}\n{}\n{}\\begin{{document}}\n{}\n{}\n\\end{{document}}",
            dc, pkgs, cmd, gpath, md, body
        )
    }
}
impl Document {
    pub fn new(class: DocumentClass) -> Self {
        Self {
            // document_class: class,
            packages: vec![],
            metadata: Metadata::new(class, "title", &["author"]),
            components: vec![],
            commands: HashMap::new(),
            img: false,
            scratch: false,
            graphics_path: None,
        }
    }

    pub fn get_command(&self, cmd: &str) -> Res<Command> {
        match self.commands.get(cmd) {
            Some(s) => Ok(s.clone()),
            None => Err(TexError::Undefined.into()),
        }
    }

    pub fn scratch(&mut self) {
        self.scratch = true;
    }

    pub fn new_command(&mut self, c: Command) {
        self.commands.insert(c.name.clone(), c);
    }

    pub fn new_component(&mut self, new: Component) {
        self.components.push(new);
    }

    pub fn set_md(&mut self, title: &str, author: &[&str]) {
        self.metadata.title = title.to_string();
        self.metadata.author = author.iter().map(|x| x.to_string()).collect();
    }

    pub fn new_package(&mut self, new: Package) {
        self.packages.push(new);
    }

    pub fn enable_graphicx(&mut self, path: &str) {
        self.img = true;
        self.new_package(package!("graphicx"));
        self.graphics_path = Some(vec![path.to_string()]);
    }

    pub fn push_gpath(&mut self, path: &str) {
        self.graphics_path.as_mut().unwrap().push(path.to_string());
    }
}
impl Opt for Document {
    fn add_option(&mut self, opt: &str) {
        self.metadata.class.add_option(opt);
    }
}
impl Populate for Document {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.new_component(other);
        Ok(self)
    }

    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        for i in other {
            self.attach(i)?;
        }
        Ok(self)
    }
}
