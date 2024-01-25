use crate::*;
use std::fs;
use std::io::Write;

#[test]
fn primary() -> Null {
    let mut q = fs::File::create("tex/primary.tex")?;
    let mut doc = Document::new(DocumentClass::new("article"));
    doc.set_md("title", "author");
    let mut p1 = Package::new("parskip");
    p1.add_option("parfill");
    doc.new_package(p1);

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn secondary() -> Null {
    // let mut q = fs::File::create("tex/secondary.tex")?;
    let mut q = fs::File::options().write(true).open("tex/secondary.tex")?;
    let mut doc = Document::new(DocumentClass::new("book"));
    doc.set_md("title", "author");
    let mut p1 = Package::new("parskip");
    p1.add_option("parfill");
    doc.new_package(p1);

    let mut part1 = Part::new("test0");
    // part1.attach(Component::Chapter(Chapter::new("testch")))?;

    let mut chap1 = Chapter::new("test1");

    let mut sec1 = Section::new("test2");

    let mut para1 = Paragraph::new();

    let mut l1 = List::new(ListType::Itemize);
    let mut l2 = List::new(ListType::Enumerate);

    let tx = TextChunk::new("This is a test sentence.", TextType::Normal);

    l1.attach(Component::TextChunk(tx.clone()))?;
    l1.attach(Component::TextChunk(tx.clone()))?;

    l2.attach(Component::TextChunk(tx.clone()))?;
    l2.attach(Component::TextChunk(tx.clone()))?;

    para1.attach(Component::List(l1.clone()))?;
    para1.attach(Component::List(l2.clone()))?;

    sec1.attach(Component::Paragraph(para1.clone()))?;
    sec1.attach(Component::Paragraph(para1.clone()))?;

    chap1.attach(Component::Section(sec1.clone()))?;
    chap1.attach(Component::Section(sec1.clone()))?;

    part1.attach(Component::Chapter(chap1.clone()))?;
    part1.attach(Component::Chapter(chap1.clone()))?;

    doc.new_component(Component::Part(part1.clone()));
    doc.new_component(Component::Part(part1.clone()));

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn tertiary() -> Null {
    let mut q = fs::File::create("tex/tertiary.tex")?;
    // let mut q = fs::File::options().write(true).open("tex/tertiary.tex")?;
    let mut doc = Document::new(DocumentClass::new("book"));
    doc.set_md("title", "author");
    let mut p1 = Package::new("parskip");
    p1.add_option("parfill");
    doc.new_package(p1);

    let mut p1 = Part::new("one");
    p1.attach(Component::Chapter(Chapter::new("c1")))?
        .attach(Component::Chapter(Chapter::new("c2")))?;

    doc.new_component(Component::Part(p1));

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn quaternary() -> Null {
    // let mut q = fs::File::create("tex/quaternary.tex")?;
    let mut q = fs::File::options().write(true).open("tex/quaternary.tex")?;
    let mut doc = Document::new(DocumentClass::new("book"));
    let mut p1 = Part::new("part1");
    p1.attach_vec(vec![Component::Chapter(Chapter::new("chap")); 5])?;
    doc.new_component(Component::Part(p1));

    doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));
    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn fifth() -> Null {
    let mut q = fs::File::create("tex/fifth.tex")?;
    // let mut q = fs::File::options().write(true).open("tex/quaternary.tex")?;
    let mut doc = Document::new(DocumentClass::new("book"));
    let mut p1 = part!("one");
    p1.attach_vec(vec![chapter!("chap"); 5])?;
    doc.new_component(p1);

    // doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));
    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn sixth() -> Null {
    let mut q = fs::File::create("tex/sixth.tex")?;
    // let mut q = fs::File::options().write(true).open("tex/quaternary.tex")?;
    let mut doc = Document::new(DocumentClass::new("article"));
    doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));
    let mut p1 = section!("one");
    p1.attach(Component::Command(
        doc.get_command("brak")?.call(vec!["hello"])?,
    ))?;
    p1.attach(command!(doc, "brak", "there"))?;
    // General Kenobi
    doc.new_component(p1);
    doc.new_package(package!("parskip", "parfill"));

    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn seventh() -> Null {
    let mut q = fs::File::create("tex/seventh.tex")?;
    // let mut q = fs::File::options().write(true).open("tex/quaternary.tex")?;
    let mut doc = document!("");
    // doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));
    // let mut p1 = section!("one");
    // p1.attach(Component::Command(
    //     doc.get_command("brak")?.call(vec!["hello"])?,
    // ))?;
    // p1.attach(command!(doc, "brak", "there"))?;
    // // General Kenobi
    // doc.new_component(p1);
    doc.new_package(package!("parskip", "parfill"));
    doc.enable_graphicx("../img");
    doc.new_component(image!("Screenshot 2023-07-15 at 1.42.09 PM.png"));
    let h = row!("a", "b", "c");
    let mut t = tabular!(3, h);
    let r = row!("d", "e", "f");
    t.attach(Component::Row(r))?;
    doc.new_component(t);
    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}

#[test]
fn eighth() -> Null {
    let mut q = fs::File::create("tex/eighth.tex")?;
    // let mut q = fs::File::options().write(true).open("tex/quaternary.tex")?;
    let mut doc = document!("");
    doc.scratch();
    // doc.attach(Component::Builtin(Builtin::new(BuiltinType::Character("infty".to_string()))))?;
    doc.attach(textchunk!(
        &builtin!(BuiltinType::Character("phi".to_string())).to_string(),
        "inline"
    ))?;
    writeln!(q, "{}", doc.to_string())?;

    Ok(())
}
