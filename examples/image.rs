use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

/// This example embeds the screenshot of this file in the pdf.
fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/with_image.tex")?;
    // We can use the document! macro to create a document, resulting in more concise code.
    // texas supports multiple document classes
    let mut doc = document!("amsart");

    // We can add things to a document using the attach method from the Populate trait.
    doc.attach(image!("examples/img/ss.png"))?;

    write!(q, "{}", doc.to_string())?;

    Ok(())
}
