use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

use crate::engine::scan;
use crate::engine::parser;

pub struct Amps {
    template: Option<String>,
    errors: Vec<String>,
}

impl Amps {
    pub fn new() -> Amps {
        Amps {
            template: None,
            errors: Vec::new(),
        }
    }

    pub fn render(&mut self) {
        let metainfo = match &self.template {
            Some(tpl) => scan::scanner::scan(&tpl),
            None      => {
                self.errors.push(String::from("no template available"));
                return;
            },
        };

        //for info in metainfo {
        //    println!("{:#?}", info);
        //}

        parser::parsing::parse(&metainfo);
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn get_template(&self) -> &Option<String> {
        &self.template
    }

    pub fn load_template(&mut self, template: String) {
        self.template = Some(template);
    }

    pub fn load_template_from_file(&mut self, filename: &String) {
        let file = File::open(&filename.trim());
        let mut file = match file {
            Ok(file)   => file,

            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    let errmsg = String::from(format!("template \"{}\" not found", filename));
                    self.errors.push(errmsg);
                    return;
                },
                ErrorKind::PermissionDenied => {
                    let errmsg = String::from(format!("lack privilege to open template {}", filename));
                    self.errors.push(errmsg);
                    return;
                },
                _ => {
                    let errmsg = String::from(format!("unexpected error opening template {}: {}",
                                                      filename, error));
                    self.errors.push(errmsg);
                    return;
                },
            }
        };

        let mut result = String::new();
        match file.read_to_string(&mut result) {
            Ok(_) => self.template = Some(result),
            Err(e) => {
                let errmsg = String::from(format!("unexpected error reading template {}: {}",
                                          filename, e));
                self.errors.push(errmsg);
            }
        }
    }
}
