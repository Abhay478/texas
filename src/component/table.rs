use itertools::Itertools;

use crate::prelude::*;

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
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.cells.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> TexResult<&mut Self> {
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
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.rows.push(other);
        Ok(self)
    }
    fn attach_vec(&mut self, mut other: Vec<Component>) -> TexResult<&mut Self> {
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
