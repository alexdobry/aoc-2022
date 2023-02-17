use std::collections::HashMap;
use std::fs;

type Path = Vec<String>;

#[derive(Debug)]
enum FileOrDir {
    File(i32),
    Dir(String),
}

type FS = HashMap<Path, Vec<FileOrDir>>;

#[derive(Debug)]
enum Instruction {
    CdUp,
    CdDown(String),
    Ls(Vec<FileOrDir>),
}

fn read_input() -> Vec<Instruction> {
    let file_path = "inputs/day07.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut cmds = input.split('$');
    let mut instructions = vec![];
    cmds.next();
    for cmd in cmds {
        let cmd = cmd.trim();
        if let Some(dir) = cmd.strip_prefix("cd ") {
            if dir == ".." {
                instructions.push(Instruction::CdUp);
            } else {
                instructions.push(Instruction::CdDown(dir.to_string()));
            }
        } else if let Some(listing) = cmd.strip_prefix("ls\n") {
            let mut entries = vec![];
            for line in listing.lines() {
                let (lhs, rhs) = line.split_once(' ').unwrap();
                if lhs == "dir" {
                    entries.push(FileOrDir::Dir(rhs.to_string()));
                } else {
                    let size = lhs.parse().unwrap();
                    entries.push(FileOrDir::File(size));
                }
            }
            instructions.push(Instruction::Ls(entries));
        } else {
            panic!("invalid command {}", cmd);
        }
    }
    instructions
}

fn make_fs(instructions: Vec<Instruction>) -> FS {
    let mut fs: FS = HashMap::new();
    let mut current_dir = vec![];
    for instr in instructions {
        match instr {
            Instruction::CdUp => {
                current_dir.pop();
            }
            Instruction::CdDown(dir) => {
                current_dir.push(dir);
            }
            Instruction::Ls(entries) => {
                fs.insert(current_dir.clone(), entries);
            }
        }
    }
    fs
}

fn size_of_dir(fs: &FS, dir: &Path) -> i32 {
    let entries = fs.get(dir).expect("expected dir to exist");
    let mut acc = 0;
    for entry in entries {
        match entry {
            FileOrDir::File(size) => {
                acc += size;
            }
            FileOrDir::Dir(path) => {
                let mut full_path = dir.clone();
                full_path.push(path.clone());
                acc += size_of_dir(fs, &full_path);
            }
        }
    }
    acc
}

pub fn solve1() {
    let instructions = read_input();
    let fs = make_fs(instructions);
    let mut result = 0;
    for path in fs.keys() {
        let size = size_of_dir(&fs, path);
        if size <= 100_000 {
            result += size;
        }
    }
    println!("{result}")
}

pub fn solve2() {
    let instructions = read_input();
    let fs = make_fs(instructions);
    let total_space = 70_000_000;
    let required_space = 30_000_000;
    let used_space = size_of_dir(&fs, &vec!["/".to_string()]);
    let unused_space = total_space - used_space;
    let needed_space = required_space - unused_space;
    let mut result = used_space;
    for path in fs.keys() {
        let size = size_of_dir(&fs, path);
        if size >= needed_space && size < result {
            result = size;
        }
    }
    println!("{result}")
}
