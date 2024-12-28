use crate::prelude::*;

// Re-exports for compatibility.
pub use beamer::*;
pub use builtin::*;
pub use envs::*;
pub use hierarchy::*;
pub use image::*;
// use markdown::mdast::Node;
pub use misc::*;
pub use table::*;
pub use textchunk::*;

/// One of the main structs, almost everything you put into a document is a `Component`
#[derive(Debug, Clone)]
pub enum Component {
    Part(Part),
    Chapter(Chapter),
    Section(Section),
    Subsection(Subsection),
    Paragraph(Paragraph),
    Line(Line),

    Frame(Frame),
    Block(Block),

    Input(Input),

    Environment(Environment),
    List(List),
    Figure(Figure),

    TextChunk(TextChunk),
    // #[cfg(feature = "markdown")]
    // Markdown(MarkdownChunk),
    Command(String),

    /// Outside the figure environment. Sometimes useful.
    Image(Image),

    Table(Table),
    Row(Row),

    Builtin(Builtin),

    Label(Label),
    Reference(Reference),
    // Dummy(Vec<Component>)
}

pub mod beamer;
pub mod builtin;
pub mod envs;
pub mod hierarchy;
pub mod image;
pub mod misc;
pub mod table;
pub mod textchunk;

impl Component {
    pub fn rank(&self) -> u8 {
        match &self {
            Component::Part(_) => 0,
            Component::Chapter(_) => 1,
            Component::Section(_) => 2,
            // #[cfg(feature = "markdown")]
            // Component::Markdown(_) => 2,
            Component::Subsection(_) => 3,
            Component::Paragraph(_) => 5,
            Component::Line(_) => 10,
            Component::Frame(_) => 4,
            Component::Block(_) => 5,

            Component::Input(_) => 9,

            Component::Environment(_) => 8,
            Component::List(_) => 7,
            Component::Figure(_) => 8,

            Component::TextChunk(_) => 10,

            Component::Command(_) => 10,

            Component::Image(_) => 10,

            Component::Table(_) => 7,

            Component::Row(_) => 10,

            Component::Builtin(_) => 10,
            Component::Label(_) => 10,
            Component::Reference(_) => 10,
        }
    }
}
impl AsLatex for Component {
    fn to_string(&self) -> String {
        match &self {
            Component::Part(stuff) => stuff.to_string(),
            Component::Chapter(stuff) => stuff.to_string(),
            Component::Section(stuff) => stuff.to_string(),
            Component::Frame(stuff) => stuff.to_string(),
            Component::Block(stuff) => stuff.to_string(),
            Component::Paragraph(stuff) => stuff.to_string(),
            Component::Line(stuff) => stuff.to_string(),
            Component::Input(stuff) => stuff.to_string(),
            Component::Environment(stuff) => stuff.to_string(),
            Component::List(stuff) => stuff.to_string(),
            Component::TextChunk(stuff) => stuff.to_string(),
            Component::Command(stuff) => stuff.to_string(),
            Component::Subsection(stuff) => stuff.to_string(),
            Component::Image(stuff) => stuff.to_string(),
            Component::Row(stuff) => stuff.to_string(),
            Component::Table(stuff) => stuff.to_string(),
            Component::Builtin(stuff) => stuff.to_string(),
            Component::Figure(stuff) => stuff.to_string(),
            Component::Label(stuff) => stuff.to_string(),
            Component::Reference(stuff) => stuff.to_string(),
            // #[cfg(feature = "markdown")]
            // Component::Markdown(stuff) => stuff.to_string(),
        }
    }
}
impl Populate for Component {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        // assert!(self.rank() >= other.rank());
        if self.rank() > other.rank() {
            return Err(TexError::RankMismatch(other.rank(), self.rank()).into());
        }
        match self {
            Component::Part(stuff) => {
                stuff.attach(other)?;
            }
            Component::Chapter(stuff) => {
                stuff.attach(other)?;
            }
            Component::Section(stuff) => {
                stuff.attach(other)?;
            }
            Component::Frame(stuff) => {
                stuff.attach(other)?;
            }
            Component::Block(stuff) => {
                stuff.attach(other)?;
            }
            Component::Subsection(stuff) => {
                stuff.attach(other)?;
            }
            Component::Paragraph(stuff) => {
                stuff.attach(other)?;
            }
            Component::Line(stuff) => {
                stuff.attach(other)?;
            }
            Component::Environment(stuff) => {
                stuff.attach(other)?;
            }
            Component::List(stuff) => {
                stuff.attach(other)?;
            }
            Component::TextChunk(stuff) => {
                stuff.attach(other)?;
            }
            Component::Row(stuff) => {
                stuff.attach(other)?;
            }
            Component::Table(stuff) => {
                stuff.attach(other)?;
            }
            _ => {
                return Err(TexError::TraitUnimplemented(format!("{:?}", &self)).into());
            }
        };

        Ok(self)
    }

    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        let q = other.iter().map(|x| x.rank()).max().unwrap();
        if self.rank() > q {
            // dbg!(self.rank());
            // dbg!(q);

            return Err(TexError::RankMismatch(q, self.rank()).into());
        }
        match self {
            Component::Part(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Chapter(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Section(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Frame(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Block(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Paragraph(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Line(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Environment(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::List(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::TextChunk(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Subsection(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Row(stuff) => {
                stuff.attach_vec(other)?;
            }
            Component::Table(stuff) => {
                stuff.attach_vec(other)?;
            }
            _ => {
                return Err(TexError::TraitUnimplemented(format!("{:?}", &self)).into());
            }
        };

        Ok(self)
    }

    fn attach_iter<I: Iterator<Item = Component>>(&mut self, mut other: I) -> TexResult<&mut Self> {
        // let q = other.map(|x| x.rank()).max().unwrap();
        if other.any(|x| x.rank() < self.rank()) {
            // dbg!(self.rank());
            // dbg!(q);

            return Err(TexError::RankMismatch(
                other.max_by_key(|x| x.rank()).unwrap().rank(),
                self.rank(),
            )
            .into());
        }
        match self {
            Component::Part(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Chapter(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Section(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Frame(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Block(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Paragraph(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Line(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Environment(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::List(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::TextChunk(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Subsection(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Row(stuff) => {
                stuff.attach_iter(other)?;
            }
            Component::Table(stuff) => {
                stuff.attach_iter(other)?;
            }
            _ => {
                return Err(TexError::TraitUnimplemented(format!("{:?}", &self)).into());
            }
        };

        Ok(self)
    }
}
