use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

/// This example embeds the screenshot of this file in the pdf.
fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/options.tex")?;
    let mut doc = document!("amsart");
    doc.disable_hyperref();

    // Add a directory in the graphics path for graphicx
    doc.push_gpath("../img");

    // Create a new Image. The macro image! gives us a Component, but we want the inner struct.
    let mut img = Image::new("ss.png");

    // Add the option!
    img.add_option("scale = 0.5");

    // Include the image in a figure enviroment.
    let mut ss = Figure::from_img(img, "this is a caption".to_string());
    ss.add_option("h");

    // attach_vec, again from the Populate trait, makes attaching multiple objects much more convenient.
    doc.attach_vec(vec![Component::Figure(ss); 3])?;

    write!(q, "{}", doc.to_string())?;

    Ok(())
}
