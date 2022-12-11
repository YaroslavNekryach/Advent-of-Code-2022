use super::super::super::utils::Result;
use crate::days::day7::folder::Folder;
use crate::days::day7::instruction::Instruction;
use crate::utils::Day;
use std::collections::HashMap;

pub struct Day7 {}

impl Day7 {
    const DAY: u8 = 7;
}

impl Day<Vec<Instruction>> for Day7 {
    fn day() -> u8 {
        Self::DAY
    }

    fn parse(input: &str) -> Result<Vec<Instruction>> {
        Ok(input.lines().map(Instruction::from_sting).collect())
    }

    fn part1(input: Vec<Instruction>) -> Result<String> {
        let base_folder = Folder::new_base();
        let mut location = base_folder.clone().full_name;
        let mut folder_list: HashMap<String, Folder> =
            HashMap::from([(base_folder.clone().full_name, base_folder)]);
        for instruction in input {
            match instruction {
                Instruction::Base => (),
                Instruction::Cd(name) => location += &format!("{name}/"),
                Instruction::Back => {
                    location = folder_list
                        .get(location.as_str())
                        .unwrap()
                        .clone()
                        .parent
                        .unwrap()
                }
                Instruction::File(file) => folder_list
                    .get_mut(location.as_str())
                    .unwrap()
                    .add_file(file),
                Instruction::Dir(name) => {
                    let folder = Folder::new(&name, &location);
                    let full_name = folder.clone().full_name;
                    folder_list.insert(full_name.clone(), folder);
                    folder_list
                        .get_mut(location.as_str())
                        .unwrap()
                        .folders
                        .push(full_name)
                }
                Instruction::List => (),
            }
        }

        let mut queue: Vec<String> = folder_list
            .values()
            .filter(|v| v.folders.is_empty())
            .map(|v| v.full_name.clone())
            .collect();

        while !queue.is_empty() {
            let full_name = queue.remove(0);
            let folder_list_clone = folder_list.clone();
            let folder = folder_list.get_mut(&full_name).unwrap();
            if folder.size.is_some() {
                continue;
            };
            let file_size: u64 = folder.files.iter().map(|v| v.size).sum();
            let folder_size: Option<u64> = folder
                .folders
                .iter()
                .map(|name| folder_list_clone.get(name).unwrap().size)
                .sum();
            if folder_size.is_none() {
                continue;
            }
            folder.size = Some(folder_size.unwrap() + file_size);
            if let Some(parent) = folder.clone().parent {
                queue.push(parent)
            }
        }
        let result = folder_list
            .values()
            .map(|folder| folder.size.unwrap())
            .filter(|size| *size <= 100000)
            .sum::<u64>();
        Ok(result.to_string())
    }

    fn part2(input: Vec<Instruction>) -> Result<String> {
        let base_folder = Folder::new_base();
        let mut location = base_folder.clone().full_name;
        let mut folder_list: HashMap<String, Folder> =
            HashMap::from([(base_folder.clone().full_name, base_folder)]);
        for instruction in input {
            match instruction {
                Instruction::Base => (),
                Instruction::Cd(name) => location += &format!("{name}/"),
                Instruction::Back => {
                    location = folder_list
                        .get(location.as_str())
                        .unwrap()
                        .clone()
                        .parent
                        .unwrap()
                }
                Instruction::File(file) => folder_list
                    .get_mut(location.as_str())
                    .unwrap()
                    .add_file(file),
                Instruction::Dir(name) => {
                    let folder = Folder::new(&name, &location);
                    let full_name = folder.clone().full_name;
                    folder_list.insert(full_name.clone(), folder);
                    folder_list
                        .get_mut(location.as_str())
                        .unwrap()
                        .folders
                        .push(full_name)
                }
                Instruction::List => (),
            }
        }

        let mut queue: Vec<String> = folder_list
            .values()
            .filter(|v| v.folders.is_empty())
            .map(|v| v.full_name.clone())
            .collect();

        while !queue.is_empty() {
            let full_name = queue.remove(0);
            let folder_list_clone = folder_list.clone();
            let folder = folder_list.get_mut(&full_name).unwrap();
            if folder.size.is_some() {
                continue;
            };
            let file_size: u64 = folder.files.iter().map(|v| v.size).sum();
            let folder_size: Option<u64> = folder
                .folders
                .iter()
                .map(|name| folder_list_clone.get(name).unwrap().size)
                .sum();
            if folder_size.is_none() {
                continue;
            }
            folder.size = Some(folder_size.unwrap() + file_size);
            if let Some(parent) = folder.clone().parent {
                queue.push(parent)
            }
        }
        let need = 30000000 - (70000000 - folder_list.get("/").unwrap().size.unwrap());
        println!("need {}", need);

        let result = folder_list
            .values()
            .map(|folder| folder.size.unwrap())
            .filter(|size| *size >= need)
            .min()
            .unwrap();
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() -> Result<()> {
        let parsed_input = Day7::parse(INPUT)?;
        let result = Day7::part1(parsed_input)?;
        assert_eq!(&result, "95437");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let parsed_input = Day7::parse(INPUT)?;
        let result = Day7::part2(parsed_input)?;
        assert_eq!(&result, "24933642");
        Ok(())
    }

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
}
