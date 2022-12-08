pub mod instructions;
pub mod parser;

use instructions::{DirName, Instruction, ListOutput};

use std::collections::BTreeMap;

pub enum FileEntry {
    Dir { name: String },
    File { name: String, size: u64 },
}

impl From<ListOutput> for FileEntry {
    fn from(list_output: ListOutput) -> Self {
        match list_output {
            ListOutput::Dir(name) => Self::Dir { name },
            ListOutput::File(size, name) => Self::File { name, size },
        }
    }
}

/// A pair of (name, size)
type FileSystemEntry = (String, u64);

fn sum_size(entries: &[FileSystemEntry]) -> u64 {
    entries.iter().map(|(_, size)| *size).sum::<u64>()
}

#[derive(Debug, Clone)]
pub struct FileSystem(BTreeMap<Vec<String>, Vec<FileSystemEntry>>);

impl FileSystem {
    pub fn sizes(&self) -> BTreeMap<Vec<String>, u64> {
        self.0.iter().fold(
            BTreeMap::<Vec<String>, u64>::new(),
            |mut acc, (dir, files)| {
                let size = sum_size(files);
                for i in 0..=dir.len() {
                    acc.entry(dir[0..i].to_vec().clone())
                        .and_modify(|v| *v += size)
                        .or_insert(size);
                }
                acc
            },
        )
    }
}

impl From<Vec<Instruction>> for FileSystem {
    fn from(instructions: Vec<Instruction>) -> Self {
        let (_, map): (Vec<String>, BTreeMap<Vec<String>, Vec<FileSystemEntry>>) =
            instructions.into_iter().fold(
                (vec![], BTreeMap::new()),
                |(mut context, mut map), instruction| {
                    match instruction {
                        Instruction::ChangeDir(DirName::Root) => {
                            context.push("".to_string());
                        }
                        Instruction::ChangeDir(DirName::Parent) => {
                            context.pop();
                        }
                        Instruction::ChangeDir(DirName::Name(target)) => {
                            context.push(target);
                        }
                        Instruction::List(list_outputs) => {
                            for file_entry in list_outputs {
                                let file_entry: FileEntry = file_entry.into();
                                if let FileEntry::File { name, size } = file_entry {
                                    map.entry(context.clone())
                                        .and_modify(|vec| vec.push((name.clone(), size)))
                                        .or_insert_with(|| vec![(name, size)]);
                                }
                            }
                        }
                    };
                    (context, map)
                },
            );
        dbg!(map.clone());
        Self(map)
    }
}
