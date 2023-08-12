use crate::*;

impl From<&str> for TextType {
    fn from(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "italic" => Self::Italic,
            "bold" => Self::Bold,
            "underline" => Self::Underlined,
            "inline" => Self::InlineMath,
            "display" => Self::DisplayMath,
            "scope" => Self::Scope,
            _ => Self::Normal
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