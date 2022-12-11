use crate::days::day7::file::File;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub full_name: String,
    pub parent: Option<String>,
    pub files: Vec<File>,
    pub folders: Vec<String>,
    pub size: Option<u64>,
}

impl Folder {
    pub fn new(name: &str, parent: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: Some(parent.to_string()),
            full_name: format!("{parent}{name}/"),
            files: Vec::new(),
            folders: Vec::new(),
            size: None,
        }
    }

    pub fn new_base() -> Self {
        Self {
            name: "/".to_string(),
            parent: None,
            full_name: "/".to_string(),
            files: Vec::new(),
            folders: Vec::new(),
            size: None,
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}
