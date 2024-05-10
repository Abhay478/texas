/// Sugar for \part{} creation
#[macro_export]
macro_rules! part {
    ($i:literal) => {
        Component::Part(Part::new($i))
    };
}

/// Sugar for \chapter{} creation
#[macro_export]
macro_rules! chapter {
    ($i:literal) => {
        Component::Chapter(Chapter::new($i))
    };
}

/// Sugar for image creation
#[macro_export]
macro_rules! image {
    ($i:literal) => {
        Component::Image(Image::new($i))
    };
}

/// Sugar for \section{} creation
#[macro_export]
macro_rules! section {
    ($i:literal) => {
        Component::Section(Section::new($i))
    };
}

/// Sugar for environment creation.
#[macro_export]
macro_rules! environment {
    ($i:literal) => {
        Component::Environment(Environment::new($i))
    };
}

/// This could've gotten real ugly if you had to do it yourself.
/// So whenever you've got a latex macro you defined earlier, and want to use it, use this macro.
/// ```rust
/// use rust_texas::*;
/// fn dummy() -> Result<(), Box<dyn std::error::Error>> {
///     let mut doc = document!("article");
///     command!(doc, "<your command/macro name>", "<appropriate arguments to the command>");
///     Ok(())
/// }
/// ```
///
/// doc-tests need all the stuff above to pass
#[macro_export]
macro_rules! command {
    ($doc:ident, $cmd:literal, $( $x:expr ),*) => {
        Component::Command($doc.get_command($cmd)?.call(vec![$($x, )*])?)
    };
}

/// Sugar for document creation
#[macro_export]
macro_rules! document {
    ($l:literal) => {
        Document::new(DocumentClass::new($l))
    };
}

/// This, too, could've gotten ugly.
/// Package creation with options.
/// ```rust
/// use rust_texas::*;
/// package!("amsmath", "<whatever options you want, as literals>");
/// ```
#[macro_export]
macro_rules! package {
    ($pkg:literal$(,)? $( $opt:literal ),*) => {{
        let mut package = Package::new($pkg); // You might get a warning here, please ignore.
        $(package.add_option($opt);)*
        package
    }};
}

/// StackOverflow: https://stackoverflow.com/a/58243493
#[macro_export]
macro_rules! unwrap {
    ($enum:path, $expr:expr) => {{
        if let $enum(item) = $expr {
            item
        } else {
            panic!()
        }
    }};
}

/// Basically `vec![]`
#[macro_export]
macro_rules! row {
    ( $( $item:literal ),* ) => {{
        let mut r = Row::new();
        $(r.attach(textchunk!($item, ""))?;)*
        r
    }};
    ( $( $item:ident ),* ) => {{
        let mut r = Row::new();
        $(r.attach($item);)*
        r
    }};
}

/// Convenient, I hope?
#[macro_export]
macro_rules! textchunk {
    ($txt:expr, $mode:literal) => {{
        let typ = TextType::from($mode);
        Component::TextChunk(TextChunk::new($txt, typ))
    }};
}

/// Sugar for table creation.
/// Different name coz `diesel` uses the `table!()` macro to do some wizardry.
#[macro_export]
macro_rules! tabular {
    ($i:literal, $h:ident) => {
        Component::Table(Table::new($i, $h))
    };
}

// Cannot think of a way to do this cleanly. Ideas would be nice. Hit up the issues page.
#[macro_export]
macro_rules! builtin {
    ($arg:expr) => {
        Component::Builtin(Builtin::new($arg))
    };
}

#[macro_export]
macro_rules! frame {
    ($title:expr) => {
        Component::Frame(Frame::new($title))
    };

}
