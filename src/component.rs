use core::str;
use std::{fmt::Display, fs::File, io::Read};
use itertools::Itertools;
use crate::*;

/// One of the main structs, almost everything you put into a document is a `Component`
#[derive(Debug, Clone)]
pub enum Component {
    Part(Part),
    Chapter(Chapter),
    Section(Section),
    Subsection(Subsection),
    Frame(Frame),
    Block(Block),
    Paragraph(Paragraph),
    Line(Line),
    Input(Input),
    Environment(Environment),
    List(List),
    TextChunk(TextChunk),
    Command(String),
    Image(Image),
    Table(Table),
    Row(Row),
    Builtin(Builtin),
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> Res<&mut Self> {
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> Res<&mut Self> {
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
pub struct Builtin {
    typ: BuiltinType,
}
impl AsLatex for Builtin {
    fn to_string(&self) -> String {
        format!("{}", self.typ.to_string())
    }
}
impl Builtin {
    pub fn new(typ: BuiltinType) -> Self {
        Self { typ }
    }
}
#[derive(Debug, Clone)]
pub enum BuiltinType {
    EnsureMath(TextChunk),
    Sin(TextChunk),
    Cos(TextChunk),
    Tan(TextChunk),
    Log(TextChunk),
    Ln(TextChunk),
    Lg(TextChunk),
    Sum(TextChunk, TextChunk),
    Prod(TextChunk, TextChunk),
    Arg(TextChunk),
    Min(TextChunk),
    Max(TextChunk),
    Character(String), // As in greek, but also stuff like \infty, etc.
}
impl Display for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::EnsureMath(s) => format!("\\ensuremath{{{}}}", s.to_string()),
                Self::Sin(s) => format!("\\sin{{{}}}", s.to_string()),
                Self::Cos(s) => format!("\\cos{{{}}}", s.to_string()),
                Self::Tan(s) => format!("\\tan{{{}}}", s.to_string()),
                Self::Log(s) => format!("\\log{{{}}}", s.to_string()),
                Self::Ln(s) => format!("\\ln{{{}}}", s.to_string()),
                Self::Lg(s) => format!("\\lg{{{}}}", s.to_string()),
                Self::Sum(down, up) =>
                    format!("\\sum_{{{}}}^{{{}}}", down.to_string(), up.to_string()),
                Self::Prod(down, up) =>
                    format!("\\prod_{{{}}}^{{{}}}", down.to_string(), up.to_string()),
                Self::Arg(s) => format!("\\arg{{{}}}", s.to_string()),
                Self::Min(s) => format!("\\min{{{}}}", s.to_string()),
                Self::Max(s) => format!("\\max{{{}}}", s.to_string()),
                Self::Character(s) => format!("\\{}", s),
            }
        )?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    cells: Vec<Component>,
}
impl AsLatex for Row {
    fn to_string(&self) -> String {
        format!(
            "{} \\\\ \n",
            self.cells.iter().map(|x| x.to_string()).join(" & ")
        )
    }
}
impl Populate for Row {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.cells.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> Res<&mut Self> {
        self.cells.append(&mut other);
        Ok(self)
    }
}
impl Row {
    pub fn new() -> Self {
        Self { cells: vec![] }
    }

    pub fn with_cells(cells: Vec<Component>) -> Self {
        Self { cells }
    }
}
/// Tables!
#[derive(Debug, Clone)]
pub struct Table {
    col: usize,
    rows: Vec<Component>,
    head: Row,
}
impl AsLatex for Table {
    fn to_string(&self) -> String {
        let s = (0..self.col).fold("|".to_string(), |acc, _x| acc + "c|");
        let rows = self.rows.iter().map(|x| x.to_string()).collect::<String>();
        format!(
            "\\begin{{tabular}}{{{}}} \n \\hline \n {} \n \\hline \n {} \\hline \\end{{tabular}} ",
            s,
            self.head.to_string(),
            rows
        )
    }
}
impl Populate for Table {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.rows.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> Res<&mut Self> {
        self.rows.append(&mut other);
        Ok(self)
    }
}
impl Table {
    pub fn new(col: usize, head: Row) -> Self {
        Self {
            col,
            rows: vec![],
            head,
        }
    }

    pub fn with_rows(col: usize, head: Row, rows: Vec<Component>) -> Self {
        Self { col, rows, head }
    }
}

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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
        Ok(self)
    }
}
impl Part {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: name.to_string(),
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
        Ok(self)
    }
}
impl Chapter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: name.to_string(),
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
        Ok(self)
    }
}
impl Section {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: name.to_string(),
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
        Ok(self)
    }
}
impl Subsection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: vec![],
        }
    }

    pub fn with_components(name: &str, components: Vec<Component>) -> Self {
        Self {
            name: name.to_string(),
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
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
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.components.extend(other.into_iter());
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
        }
    }
}
impl Populate for TextChunk {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        if let Component::TextChunk(ch) = other {
            self.body.push_str(&ch.body);
            Ok(self)
        } else {
            Err(TexError::RankMismatch.into())
        }
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
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
    pub fn from_file(path: &str) -> Res<Self> {
        let mut f = File::open(path)?;
        let mut buf = "".to_string();
        f.read_to_string(&mut buf)?;
        Ok(Self {
            body: buf,
            typ: TextType::Normal,
        })
    }
}

/// \input{}, if you want that kinda thing. ~Personally, I've never used it.~
/// I _have_ used it. Pretty useful.
#[derive(Debug, Clone)]
pub struct Input {
    name: String,
}
impl AsLatex for Input {
    fn to_string(&self) -> String {
        format!("\\input{{{}}}", self.name)
    }
}

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
        format!(
            "\\begin{{{}}} \n {} \n \\end{{{}}} \n ",
            self.name, comps, self.name
        )
    }
}
impl Populate for Environment {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.components.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
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
}
impl AsLatex for List {
    fn to_string(&self) -> String {
        let comps = self
            .items
            .iter()
            .map(|s| format!("\t\\item {}\n", s.to_string()))
            .collect::<String>();
        format!(
            "\\begin{{{}}} \n {} \n \\end{{{}}} \n ",
            self.typ.to_string(),
            comps,
            self.typ.to_string()
        )
    }
}
impl Populate for List {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        self.items.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        self.items.extend(other.into_iter());
        Ok(self)
    }
}
impl List {
    pub fn new(typ: ListType) -> Self {
        Self {
            items: vec![],
            typ,
        }
    }

    pub fn with_items(typ: ListType, items: Vec<Component>) -> Self {
        Self { items, typ }
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
}

impl Component {
    pub fn rank(&self) -> u8 {
        match &self {
            Self::Part(_) => 0,
            Self::Chapter(_) => 1,
            Self::Section(_) => 2,
            Self::Frame(_) => 4,
            Self::Block(_) => 5,
            Self::Paragraph(_) => 5,
            Self::Line(_) => 10,
            Self::Input(_) => 9,
            Self::Environment(_) => 8,
            Self::List(_) => 7,
            Self::TextChunk(_) => 10,
            Self::Command(_) => 10,
            Self::Subsection(_) => 3,
            Self::Image(_) => 10,
            Self::Table(_) => 7,
            Self::Row(_) => 10,
            Self::Builtin(_) => 10,
        }
    }
}
impl AsLatex for Component {
    fn to_string(&self) -> String {
        match &self {
            Self::Part(stuff) => stuff.to_string(),
            Self::Chapter(stuff) => stuff.to_string(),
            Self::Section(stuff) => stuff.to_string(),
            Self::Frame(stuff) => stuff.to_string(),
            Self::Block(stuff) => stuff.to_string(),
            Self::Paragraph(stuff) => stuff.to_string(),
            Self::Line(stuff) => stuff.to_string(),
            Self::Input(stuff) => stuff.to_string(),
            Self::Environment(stuff) => stuff.to_string(),
            Self::List(stuff) => stuff.to_string(),
            Self::TextChunk(stuff) => stuff.to_string(),
            Self::Command(stuff) => stuff.to_string(),
            Self::Subsection(stuff) => stuff.to_string(),
            Self::Image(stuff) => stuff.to_string(),
            Self::Row(stuff) => stuff.to_string(),
            Self::Table(stuff) => stuff.to_string(),
            Self::Builtin(stuff) => stuff.to_string(),
        }
    }
}
impl Populate for Component {
    fn attach(&mut self, other: Component) -> Res<&mut Self> {
        // assert!(self.rank() >= other.rank());
        if self.rank() > other.rank() {
            return Err(TexError::RankMismatch.into());
        }
        match self {
            Self::Part(stuff) => {
                stuff.attach(other)?;
            }
            Self::Chapter(stuff) => {
                stuff.attach(other)?;
            }
            Self::Section(stuff) => {
                stuff.attach(other)?;
            }
            Self::Frame(stuff) => {
                stuff.attach(other)?;
            }
            Self::Block(stuff) => {
                stuff.attach(other)?;
            }
            Self::Subsection(stuff) => {
                stuff.attach(other)?;
            }
            Self::Paragraph(stuff) => {
                stuff.attach(other)?;
            }
            Self::Line(stuff) => {
                stuff.attach(other)?;
            }
            Self::Input(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Environment(stuff) => {
                stuff.attach(other)?;
            }
            Self::List(stuff) => {
                stuff.attach(other)?;
            }
            Self::TextChunk(stuff) => {
                stuff.attach(other)?;
            }
            Self::Command(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Image(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Row(stuff) => {
                stuff.attach(other)?;
            }
            Self::Table(stuff) => {
                stuff.attach(other)?;
            }
            Self::Builtin(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
        };

        Ok(self)
    }

    fn attach_vec(&mut self, other: Vec<Component>) -> Res<&mut Self> {
        let q = other.iter().map(|x| x.rank()).max().unwrap();
        if self.rank() > q {
            dbg!(self.rank());
            dbg!(q);

            return Err(TexError::RankMismatch.into());
        }
        match self {
            Self::Part(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Chapter(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Section(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Frame(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Block(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Paragraph(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Line(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Input(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Environment(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::List(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::TextChunk(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Command(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Subsection(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Image(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Row(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Table(stuff) => {
                stuff.attach_vec(other)?;
            }
            Self::Builtin(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
        };

        Ok(self)
    }
}
