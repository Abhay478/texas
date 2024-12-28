use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/command.tex")?;
    let mut doc = document!("amsart");

    // This compiles to \newcommand{}[]{} in latex. 
    doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));

    let mut pm = Environment::new("pmatrix");

    doc.new_command(Command::new("myvec", 1, &pm.attach(textchunk!("#1"))?.to_string()));

    doc.attach(command!(doc, "brak", "Hello World."))?;
    doc.attach(command!(doc, "myvec", "1 & 2 & 3"))?;

    writeln!(q, "{}", doc.to_string())?;
    Ok(())
}
