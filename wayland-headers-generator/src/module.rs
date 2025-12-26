use std::{fs::File, io::Write};

pub(crate) struct Module {
    name: String,
    pub(crate) imports: Vec<String>,
    pub(crate) structs: Vec<(String, String)>,
    pub(crate) constants: Vec<(String, String)>,
    pub(crate) functions: Vec<(String, String)>,
    pub(crate) type_aliases: Vec<(String, String)>,
}

impl Module {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            imports: Vec::new(),
            structs: Vec::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            type_aliases: Vec::new(),
        }
    }

    pub(crate) fn write_file(&mut self) {
        let path = format!("wayland-headers/src/{}.rs", self.name);
        let mut file = File::create(path).unwrap();

        self.sort_and_write_imports(&mut file);
        self.sort_and_write_structs(&mut file);
        self.sort_and_write_constants(&mut file);
        self.sort_and_write_functions(&mut file);
        self.sort_and_write_type_aliases(&mut file);
    }

    fn sort_and_write_imports(&mut self, file: &mut File) {
        self.imports.sort();
        for text in &self.imports {
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_structs(&mut self, file: &mut File) {
        self.structs.sort();
        for (_, text) in &self.structs {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_constants(&mut self, file: &mut File) {
        if self.constants.is_empty() {
            return;
        }

        writeln!(file).unwrap();

        self.constants.sort();
        for (_, text) in &self.constants {
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_functions(&mut self, file: &mut File) {
        self.functions.sort();
        for (_, text) in &self.functions {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }

    fn sort_and_write_type_aliases(&mut self, file: &mut File) {
        if self.type_aliases.is_empty() {
            return;
        }

        writeln!(file).unwrap();

        self.type_aliases.sort();
        for (_, text) in &self.type_aliases {
            writeln!(file, "{text}").unwrap();
        }
    }
}
