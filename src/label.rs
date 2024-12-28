use crate::prelude::AsLatex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Label {
    Standard(String),
    Chapter(String),
    Section(String),
    Subsection(String),
    Equation(String),
    Table(String),
    Figure(String),
    Code(String),
    Item(String),
    Algorithm(String),
}

impl AsLatex for Label {
    fn to_string(&self) -> String {
        format!(
            " \\label{{{}}} \n",
            match &self {
                Label::Standard(s) => format!("std:{s}"),
                Label::Equation(s) => format!("eq:{s}"),
                Label::Table(s) => format!("tab:{s}"),
                Label::Figure(s) => format!("fig:{s}"),
                Label::Section(s) => format!("sec:{s}"),
                Label::Subsection(s) => format!("subsec:{s}"),
                Label::Code(s) => format!("lst:{s}"),
                Label::Item(s) => format!("itm:{s}"),
                Label::Algorithm(s) => format!("alg:{s}"),
                Label::Chapter(s) => format!("ch:{s}"),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reference {
    Standard(String),
    Chapter(String),
    Section(String),
    Subsection(String),
    Equation(String),
    Table(String),
    Figure(String),
    Code(String),
    Item(String),
    Algorithm(String),
}

impl AsLatex for Reference {
    fn to_string(&self) -> String {
        format!(
            "~\\ref{{{}}} \n",
            match &self {
                Reference::Standard(s) => format!("std:{s}"),
                Reference::Equation(s) => format!("eq:{s}"),
                Reference::Table(s) => format!("tab:{s}"),
                Reference::Figure(s) => format!("fig:{s}"),
                Reference::Section(s) => format!("sec:{s}"),
                Reference::Subsection(s) => format!("subsec:{s}"),
                Reference::Code(s) => format!("lst:{s}"),
                Reference::Item(s) => format!("itm:{s}"),
                Reference::Algorithm(s) => format!("alg:{s}"),
                Reference::Chapter(s) => format!("ch:{s}"),
            }
        )
    }
}
