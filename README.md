# TEXAS

This crate used to be Texas with a capital T. An issue was raised, and thus I have 'renamed' it the only way I know how. Apologies for any inconvenience. It is now `rust-texas`.

## Purpose

### Not the purpose

This crate does not, in any way, even remotely cover the vast variety of things you can do with latex. Instead, it attempts to provide a friendly API for some of the most basic functions. Furthermore, it does not catch most latex errors.

### The actual purpose

`texas` is a crate for programmatic latex generation. Its primary intended use it to collate data from multiple sources, (e.g. text files of varying formats, sqlite databases, images, tables, a counter within your program, the network, or hardcoded strings)  into a single latex/beamer document, with an increasing number of format options. It attempts to automate data compilation with a latex or pdf target.

More concretely, it is a simple AST for a subset of latex, with the ability to compile down to a latex document.

It's also my first foray into the open-source world, so constructive criticism is welcome and appreciated. https://github.com/Abhay478/texas/issues

## Basics

`texas` creates a tree (technically, a nested enum hierarchy) of `Component`s, each of which can be converted to a string. This tree is rooted at the `Document`, from which all `Component`s are recursively converted into strings and concatenated.

- The primary type is `Document`, which you populate per your whims and fancies. This can be written to a file like so: 

```rust
let mut q = File::create("file.tex")?;
let doc = document!("book");
write!(q, "{}", doc.to_string())?
```
- The document can be filled with `Component`s (including `Label`s, `Reference`s, `Environment`s, etc.), `Package`s, and `Command`s. They can be created using both functions and macros.
- `Component` is an enum, with each variant containing a separate struct. If a component `impl`s the `Populate` trait, you can fill it with more `Component`s, then install it in the `Document` like so:

```rust
let mut p1 = part!("one");
p1.attach(chapter!("c1"))?
    .attach(chapter!("c2"))?; // and so on.

p1.attach_vec(vec![chapter!("c3"); 2])?;

doc.new_component(p1);
```
- `Command`s can be created and installed like so: 
```rust
doc.new_command(Command::new("brak", 1, r"\ensuremath{\left(#1\right)}"));
```
- And commands can be called in-text like so: 
```rust
let mut p1 = section!("one");
p1.attach(command!(doc, "brak", "hello there"))?;
```

- `Package`s can be created and installed too: 
```rust
doc.new_package(package!("parskip", "parfill"));
```
- Also has trait `Opt`, which allows for adding options to a command (like `usepackage` and `documentclass`, for now).

## Components

We have a lot of them.

### Hierarchy

These are regions in the document.

- Part
- Chapter
- Section
- Subsection
- Paragraph
- Line

### Beamer

Support for beamer has been around since 0.3.0. The following components are available:

- Frame
- Block

### Environments

Well, I haven't added all of them. You can't make your own environments (that's upcoming) but you can use any environment with the `Environment` struct.

- Environment
- List: Specialised struct for Itemize and Enumerate environments.
- Figure: Specialised struct for the Figure environment.

### Basic Text

- TextChunk: Text of several different types (normal, italic, bold, etc.). Refer the `TextType` enum for more.

### Tables

- Table
- Row: A series of TextChunks seperated by `&`. Can be used in `align` environments too. 

### Builtin

- Builtin: All the little symbols (`\phi`, `\infty`) and stuff (`\ensuremath`). Refer the `BuiltinType` enum for more.

### Labels

- Label
- Reference

### Misc

- Image
- Command
- Input

## Parser (new)

An as-of-yet unimplemented feature, `texas` will have a Markdown parser by 0.4.0. Due to inconveniences faced while attempting to adapt the ASTs produced by other parsers, namely in converting them into `texas`' `Component` tree, I decided to write *another* markdown parser.

The parser will read a markdown file and produce a `Document`. 

## Log 

- ### 0.3.5
  - Added Labels! Again, something no other Latex crate has as far as I know.
  - Made a prelude! For anything `texas`-y, just add `use rust_texas::prelude::*;`.
  - Split `component.rs` and `document.rs` into multiple files.
  - Made documents include `graphicx` and `hyperref` by default.
  - Added more `TextType`s.
  - Fixed a few bugs with `Opt`, namely that it was doing nothing for environments.
  - Other minor changes.
- ### 0.3.6
  - Added an examples directory, explaining some of the ways `texas` can be used.
  - **BREAKING:** Added a new function to the `Populate` trait, `attach_iter`. 
  - New builtin type (Surround), better macros.
  - Progress on implementing a Markdown Parser.
    - 