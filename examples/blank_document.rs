use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/blank.tex")?;
    let mut doc = Document::new(DocumentClass::new("article"));
    doc.set_md(
        "Amazing Title",
        &["Author", "Also Author", "Yet Another Author"],
    );
    // Uncomment to not include the hyperref and graphicx packages, which are included by default.
    // doc.disable_graphicx();
    // doc.disable_hyperref();
    writeln!(q, "{}", doc.to_string())?;
    Ok(())
}
