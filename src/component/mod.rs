use crate::prelude::*;

// Re-exports for compatibility.
pub use beamer::*;
pub use builtin::*;
pub use envs::*;
pub use hierarchy::*;
pub use image::*;
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

    Command(String),

    /// Outside the figure environment. Sometimes useful.
    Image(Image),

    Table(Table),
    Row(Row),

    Builtin(Builtin),

    Label(Label),
    Reference(Reference),
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
            Self::Part(_) => 0,
            Self::Chapter(_) => 1,
            Self::Section(_) => 2,
            Self::Subsection(_) => 3,
            Self::Paragraph(_) => 5,
            Self::Line(_) => 10,
            Self::Frame(_) => 4,
            Self::Block(_) => 5,

            Self::Input(_) => 9,

            Self::Environment(_) => 8,
            Self::List(_) => 7,
            Self::Figure(_) => 8,

            Self::TextChunk(_) => 10,

            Self::Command(_) => 10,

            Self::Image(_) => 10,

            Self::Table(_) => 7,

            Self::Row(_) => 10,

            Self::Builtin(_) => 10,
            Self::Label(_) => 10,
            Self::Reference(_) => 10,
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
            Self::Figure(stuff) => stuff.to_string(),
            Self::Label(stuff) => stuff.to_string(),
            Self::Reference(stuff) => stuff.to_string(),
        }
    }
}
impl Populate for Component {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
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
            Self::Figure(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Label(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Reference(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
        };

        Ok(self)
    }

    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
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
            Self::Figure(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Label(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
            Self::Reference(_) => {
                return Err(TexError::TraitUnimplemented.into());
            }
        };

        Ok(self)
    }
}
