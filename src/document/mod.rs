use std::
    collections::HashMap
;

use crate::prelude::*;

mod doc_class;
mod package;
mod metadata;

pub use doc_class::*;
pub use package::*;
pub use metadata::*;


/// The king of the land. The `Document` type is where you start.
/// Has macro support.
/// Also contains a restriction on the latex commands you use - you can't use one without
/// declaring it. Atypical of this crate, this particular feature prevents a latex error.
#[derive(Debug, Clone)]
pub struct Document {
    // document_class: DocumentClass,
    packages: Vec<Package>,
    pub metadata: Metadata,
    components: Vec<Component>,
    commands: HashMap<String, Command>,
    // labels: HashSet<&'a Label>,
    img: bool,
    href: bool,
    scratch: bool,
    graphics_path: Option<Vec<String>>,
}
impl AsLatex for Document {
    fn to_string(&self) -> String {
        let dc = self.metadata.class.to_string();
        let pkgs = self
            .packages
            .iter()
            .map(|x| x.to_string())
            .collect::<String>();
        let md = if !self.scratch {
            self.metadata.to_string()
        } else {
            "\n".to_string()
        };
        let beamer_init_frames: String = if self.metadata.class.typ == DocumentClassType::Beamer {
            // Warning: Unused result. Again, cannot n-choose-2 Component Variants.
            let title_frame = Frame::with_components("", vec![textchunk!(r"\titlepage", "normal")]);

            let mut toc = "".to_string();
            if self.metadata.tableofcontents {
                toc = Frame::with_components("", vec![textchunk!(r"\tableofcontents", "normal")])
                    .to_string();
            };

            format!("{}\n{}\n", title_frame.to_string(), toc)
        } else {
            "\n".to_string()
        };
        let body = beamer_init_frames
            + &self
                .components
                .iter()
                .map(|x| x.to_string())
                .collect::<String>();

        let cmd = self
            .commands
            .iter()
            .map(|x| format!("{} \n", x.1.declare()))
            .collect::<String>();

        let gpath = if let Some(path) = &self.graphics_path {
            format!(
                "\\graphicspath{{{}}} \n",
                path.iter()
                    .map(|x| format!("{{{}}}, ", x))
                    .collect::<String>()
            )
        } else {
            "".to_string()
        };
        format!(
            "{}\n{}\n{}\n{}\\begin{{document}}\n{}\n{}\n\\end{{document}}",
            dc, pkgs, cmd, gpath, md, body
        )
    }
}
impl Document {
    pub fn new(class: DocumentClass) -> Self {
        let mut out = Self {
            // document_class: class,
            packages: vec![],
            metadata: Metadata::new(class, "title", &["author"]),
            components: vec![],
            commands: HashMap::new(),
            // labels: HashSet::new(),
            img: true,
            href: true,
            scratch: false,
            graphics_path: None,
        };
        out.new_package(package!("graphicx"));
        out.new_package(package!("hyperref"));
        out
    }

    pub fn get_command(&self, cmd: &str) -> TexResult<Command> {
        match self.commands.get(cmd) {
            Some(s) => Ok(s.clone()),
            None => Err(TexError::Undefined.into()),
        }
    }

    pub fn scratch(&mut self) {
        self.scratch = true;
    }

    pub fn new_command(&mut self, c: Command) {
        self.commands.insert(c.name.clone(), c);
    }

    pub fn new_component(&mut self, new: Component) {
        self.components.push(new);
    }

    pub fn set_md(&mut self, title: &str, author: &[&str]) {
        self.metadata.title = title.to_string();
        self.metadata.author = author.iter().map(|x| x.to_string()).collect();
    }

    pub fn new_package(&mut self, new: Package) {
        self.packages.push(new);
    }

    pub fn enable_graphicx(&mut self, path: &str) {
        self.img = true;
        self.new_package(package!("graphicx"));
        self.graphics_path = Some(vec![path.to_string()]);
    }

    pub fn disable_graphicx(&mut self) {
        self.img = false;
        self.packages.retain(|x| x.name != "graphicx");
        self.graphics_path = None;
    }

    pub fn enable_hyperref(&mut self) {
        self.href = true;
        self.new_package(package!("hyperref"));
    }

    pub fn disable_hyperref(&mut self) {
        self.href = false;
        self.packages.retain(|x| x.name != "hyperref");
    }

    pub fn push_gpath(&mut self, path: &str) {
        self.graphics_path.as_mut().unwrap().push(path.to_string());
    }
}
impl Opt for Document {
    fn add_option(&mut self, opt: &str) {
        self.metadata.class.add_option(opt);
    }
}
impl Populate for Document {
    fn attach(&mut self, other: Component) -> TexResult<&mut Self> {
        self.new_component(other);
        Ok(self)
    }

    fn attach_vec(&mut self, other: Vec<Component>) -> TexResult<&mut Self> {
        for i in other {
            self.attach(i)?;
        }
        Ok(self)
    }
}
