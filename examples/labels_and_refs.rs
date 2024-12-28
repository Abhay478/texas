use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/l_and_r.tex")?;
    let mut doc = document!("amsart");

    doc.push_gpath("../img");

    let f1 = figure!("ss.png");
    let lb1 = label!("fig:f1");

    let f2 = figure!("ss.png");
    let lb2 = label!("fig:f2");
    
    let l1 = Line::with_components(vec![textchunk!("This is a reference: "), reference!("fig:f1")]);
    let l2 = Line::with_components(vec![textchunk!("And so is this: "), reference!("fig:f2")]);
    doc.attach(f1)?.attach(lb1)?.attach(f2)?.attach(lb2)?.attach(Component::Line(l1))?.attach(Component::Line(l2))?;

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}
