use crate::days::day7::file::File;
use regex::Regex;

#[derive(Clone)]
pub enum Instruction {
    Base,
    List,
    Cd(String),
    Dir(String),
    Back,
    File(File),
}

impl Instruction {
    pub fn from_sting(s: &str) -> Instruction {
        let init = Regex::new(r"^\$ cd /").unwrap();
        let list = Regex::new(r"^\$ ls").unwrap();
        let folder = Regex::new(r"^dir").unwrap();
        let file = Regex::new(r"^(\d+) (.+)$").unwrap();
        let back = Regex::new(r"^\$ cd \.\.").unwrap();
        let cd = Regex::new(r"^\$ cd .*").unwrap();

        if init.is_match(s) {
            Instruction::Base
        } else if list.is_match(s) {
            Instruction::List
        } else if back.is_match(s) {
            Instruction::Back
        } else if folder.is_match(s) {
            let (_, name) = s.split_at(4);
            Instruction::Dir(name.to_string())
        } else if file.is_match(s) {
            let file = File::from_string(s).unwrap();
            Instruction::File(file)
        } else if cd.is_match(s) {
            let (_, name) = s.split_at(5);
            Instruction::Cd(name.to_string())
        } else {
            panic!("Invalid instruction")
        }
    }
}
