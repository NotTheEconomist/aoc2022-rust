#[derive(Debug, Clone)]
pub enum Instruction {
    /// A "cd" command
    ChangeDir(DirName),
    /// An "ls" command. Contains its output as list of string
    List(Vec<ListOutput>),
}

/// The output from a list command
#[derive(Debug, Clone)]
pub enum ListOutput {
    Dir(String),
    File(u64, String),
}

#[derive(Debug, Clone)]
pub enum DirName {
    Root,
    Parent,
    Name(String),
}
