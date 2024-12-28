use std::error::Error;
use std::fs::File;
use std::io::Write;

use rust_texas::prelude::*;

// walkdir is a dev dependency, and you can implement custom walkdir-ish iterators for your own use case.
// I have another crate, Beary, which implements a walkdir-like iterator for the storage format of Bear, the note taking app.
use walkdir::WalkDir;

/// We'll put all of texas' example code in a pdf.
///
/// Ideally, you'd be doing this for a bunch of markdown files or text files, but
/// it seemed easier to demonstrate this way. That said, it's still a somewhat complex example.
fn main() -> Result<(), Box<dyn Error>> {
    let mut q = File::create("examples/tex/source_code.tex")?;
    let mut doc = document!("amsart");
    doc.disable_hyperref();

    // Using the listings package, because we're making a source code pdf.
    doc.new_package(Package::new("listings"));

    // The final, NEW, function from the Populate trait is attach_iter. Here, we feed it an iterator over
    // components, namely one section per file in the examples directory.
    // Because at the time of writing `examples` is flat, walkdir is overkill. However,
    // it illustrated how such walkers can be used alongside texas.
    doc.attach_iter(WalkDir::new("examples").into_iter().filter_map(|x| {
        let x = x.ok()?;
        if x.metadata().unwrap().is_dir() {
            None
        } else {
            let fname = x.into_path();
            let fname = fname.to_str().unwrap();

            // This reads the contents of a file and dumps it (raw) into a TextChunk component with TextType::Normal.
            // At this stage, it's just a string.
            let buf = TextChunk::from_file(&fname).ok()?;

            // Because this is source code, we'll put it in an lstlisting environment.
            let mut vb = Environment::new("lstlisting");
            // And embed buf in the lstlisting.
            vb.attach(Component::TextChunk(buf)).ok()?;

            // We'll create a new section for each file, for no particular reason.
            let mut sec = section!(&fname);
            sec.attach(Component::Environment(vb)).ok()?;
            Some(sec)
        }
    }))?;

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}
