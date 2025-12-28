use std::{fs::File, io::Write};

pub(crate) struct Module {
    name: String,
    pub(crate) imports: Vec<String>,
    pub(crate) structs: Vec<(String, String)>,
    pub(crate) constants: Vec<(String, String)>,
    pub(crate) extern_statics: Vec<(String, String)>,
    pub(crate) functions: Vec<(String, String)>,
}

impl Module {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            imports: Vec::new(),
            structs: Vec::new(),
            constants: Vec::new(),
            extern_statics: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub(crate) fn write_file(&mut self) {
        let path = format!("wayland-headers/src/{}.rs", self.name);
        let mut file = File::create(path).unwrap();

        self.sort_and_write_imports(&mut file);
        self.sort_and_write_structs(&mut file);
        self.sort_and_write_constants(&mut file);
        self.sort_and_write_extern_statics(&mut file);
        self.sort_and_write_functions(&mut file);
    }

    fn sort_and_write_imports(&mut self, file: &mut File) {
        writeln!(file, "use crate::prelude::*;").unwrap();

        self.imports.sort();
        for text in &self.imports {
            writeln!(file).unwrap();
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

    fn sort_and_write_extern_statics(&mut self, file: &mut File) {
        if self.extern_statics.is_empty() {
            return;
        }

        writeln!(file).unwrap();
        writeln!(file, "unsafe extern \"C\" {{").unwrap();

        self.extern_statics.sort();
        for (_, text) in &self.extern_statics {
            writeln!(file, "{text}").unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    fn sort_and_write_functions(&mut self, file: &mut File) {
        self.functions.sort();
        for (_, text) in &self.functions {
            writeln!(file).unwrap();
            writeln!(file, "{text}").unwrap();
        }
    }
}
