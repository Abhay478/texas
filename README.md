# TEXAS

### This crate used to be Texas with a capital T. An issue was raised, and thus I have 'renamed' it the only way I know how. Apologies for any inconvenience. It is now `rust-texas`.

## Purpose

This crate does not, in any way, even remotely cover the vast variety of things you can do with latex. Instead, it attempts to provide a friendly API
for some of the most basic functions. Furthermore, it does not catch most latex errors.

It's also my first foray into the open-source world, so constructive criticism is welcome and appreciated. https://github.com/Abhay478/texas/issues



## Basics
- The primary type is `Document`, which you populate per your whims and fancies. This can be written to a file like so: 

```rust
let mut q = File::create("file.tex")?;
let doc = document!("book");
write!(q, "{}", doc.to_string())?
```
- The document can be filled with `Component`s, `Package`s and `Command`s. They can be created using both functions and macros.
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
doc.new_command(Command::new("brak", 1, "\\ensuremath{\\left(#1\\right)}"));
```
- And commands can be called in-text like so: 
```rust
let mut p1 = section!("one");
p1.attach(command!(doc, "brak", "hello there"))?;
```
- Will add ability to generate stuff like `ensuremath` from code eventually.
- Packages can be created and installed like so: 
```rust
doc.new_package(package!("parskip", "parfill"));
```
- Also has trait `Opt`, which allows for adding options to a command (like `usepackage` and `documentclass`, for now).

## Log 
- ### 0.2.3
  - Lil cleanup
  - Finally added the ensuremath stuff. Adding a bunch more.
- ### 0.2.[45]
  - Nothing, really. Renamed the crate. Big whoop.
- 0.2.6
  - Added a few Debug/Clone derives.