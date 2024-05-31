use crate::prelude::*;

impl From<&str> for TextType {
    fn from(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "italic" | "emph" => Self::Italic,
            "bold" | "strongemph" => Self::Bold,
            "teletype" | "tt" => Self::Teletype,
            "mathbold" | "mbf" => Self::MathBold,
            "mathcal" | "mcal" => Self::MathCal,
            "mathbb" | "mbb" => Self::MathBb,
            "mathrm" | "mrm" => Self::MathRm,
            "underline" => Self::Underlined,
            "inline" | "math" | "dollar" => Self::InlineMath,
            "display" | "dollardollar" => Self::DisplayMath,
            "scope" => Self::Scope,
            "verbatim" => Self::Verbatim,
            "strikethrough" | "ss" => Self::Strikethrough,

            _ => Self::Normal,
        }
    }
}

impl From<Part> for Component {
    fn from(value: Part) -> Self {
        Self::Part(value)
    }
}

impl From<Chapter> for Component {
    fn from(value: Chapter) -> Self {
        Self::Chapter(value)
    }
}

impl From<Section> for Component {
    fn from(value: Section) -> Self {
        Self::Section(value)
    }
}

impl From<Paragraph> for Component {
    fn from(value: Paragraph) -> Self {
        Self::Paragraph(value)
    }
}

impl From<Line> for Component {
    fn from(value: Line) -> Self {
        Self::Line(value)
    }
}

impl From<List> for Component {
    fn from(value: List) -> Self {
        Self::List(value)
    }
}

impl From<Input> for Component {
    fn from(value: Input) -> Self {
        Self::Input(value)
    }
}

impl From<TextChunk> for Component {
    fn from(value: TextChunk) -> Self {
        Self::TextChunk(value)
    }
}

impl From<Subsection> for Component {
    fn from(value: Subsection) -> Self {
        Self::Subsection(value)
    }
}

impl From<Image> for Component {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<Environment> for Component {
    fn from(value: Environment) -> Self {
        Component::Environment(value)
    }
}

/*  


    Builtin(Builtin),

    Label(Label),
    Reference(Reference), */

impl From<Frame> for Component {
    fn from(value: Frame) -> Self {
        Component::Frame(value)
    }
}

impl From<Block> for Component {
    fn from(value: Block) -> Self {
        Component::Block(value)
    }
}

impl From<Figure> for Component {
    fn from(value: Figure) -> Self {
        Component::Figure(value)
    }
}

impl From<Table> for Component {
    fn from(value: Table) -> Self {
        Component::Table(value)
    }
}

impl From<Row> for Component {
    fn from(value: Row) -> Self {
        Component::Row(value)
    }
}

impl From<Builtin> for Component {
    fn from(value: Builtin) -> Self {
        Component::Builtin(value)
    }
}
impl From<Label> for Component {
    fn from(value: Label) -> Self {
        Component::Label(value)
    }
}
impl From<Reference> for Component {
    fn from(value: Reference) -> Self {
        Component::Reference(value)
    }
}


impl From<&str> for Label {
    fn from(value: &str) -> Self {
        let q = value.find(":").unwrap_or(0);
        let typ = &value[..q];
        let lbl = &value[q + 1..];
        match typ {
            "ch" => Self::Chapter(lbl.to_string()),
            "sec" => Self::Section(lbl.to_string()),
            "subsec" => Self::Subsection(lbl.to_string()),
            "eq" => Self::Equation(lbl.to_string()),
            "tab" => Self::Table(lbl.to_string()),
            "fig" => Self::Figure(lbl.to_string()),
            "lst" => Self::Code(lbl.to_string()),
            "itm" => Self::Item(lbl.to_string()),
            "alg" => Self::Algorithm(lbl.to_string()),
            _ => Self::Standard(value.to_string()),
        }
    }
}

impl From<&str> for Reference {
    fn from(value: &str) -> Self {
        let q = value.find(":").unwrap_or(0);
        let typ = &value[..q];
        let lbl = &value[q + 1..];
        match typ {
            "ch" => Self::Chapter(lbl.to_string()),
            "sec" => Self::Section(lbl.to_string()),
            "subsec" => Self::Subsection(lbl.to_string()),
            "eq" => Self::Equation(lbl.to_string()),
            "tab" => Self::Table(lbl.to_string()),
            "fig" => Self::Figure(lbl.to_string()),
            "lst" => Self::Code(lbl.to_string()),
            "itm" => Self::Item(lbl.to_string()),
            "alg" => Self::Algorithm(lbl.to_string()),
            _ => Self::Standard(value.to_string()),
        }
    }
}
