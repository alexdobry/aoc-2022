use std::fs;

#[derive(Debug)]
struct Table {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    from: usize,
    to: usize,
}

fn parse_table(table: &str) -> Table {
    let mut res: Table = Table { stacks: vec![] };
    let (table, legend) = table.rsplit_once("\n").unwrap();
    for _ in legend.split_whitespace() {
        res.stacks.push(vec![]);
    }
    for line in table.lines().rev() {
        let bytes = line.as_bytes();
        for i in 0..res.stacks.len() {
            let char = bytes[1 as usize + i * 4] as char;
            if !char.is_whitespace() {
                res.stacks[i].push(char);
            }
        }
    }
    res
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let mut res: Vec<Instruction> = vec![];
    for line in instructions.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();
        let count: usize = parts[1].parse().unwrap();
        for _ in 0..count {
            res.push(Instruction { from, to })
        }
    }
    res
}

fn parse_instructions2(instructions: &str) -> Vec<Instruction> {
    let mut res: Vec<Instruction> = vec![];
    for line in instructions.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();
        let count: usize = parts[1].parse().unwrap();
        let tmp: usize = if 1 != from && 1 != to {
            1
        } else if 2 != from && 2 != to {
            2
        } else {
            3
        };
        for _ in 0..count {
            res.push(Instruction { from, to: tmp })
        }
        for _ in 0..count {
            res.push(Instruction { from: tmp, to })
        }
    }
    res
}

fn read_input() -> (Table, String) {
    let file_path = "inputs/day05.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (table, instructions) = contents.split_once("\n\n").unwrap();
    let table = parse_table(table);
    (table, instructions.to_owned())
}

fn interpret(table: Table, instructions: Vec<Instruction>) {
    let mut table = table;
    for instr in instructions {
        let tmp = table.stacks[instr.from - 1]
            .pop()
            .expect("tried to take from an empty stack");
        table.stacks[instr.to - 1].push(tmp);
    }
    for stack in table.stacks {
        print!("{}", stack.last().unwrap())
    }
    println!();
}

pub fn solve1() {
    let (table, instructions) = read_input();
    interpret(table, parse_instructions(&instructions));
}

pub fn solve2() {
    let (table, instructions) = read_input();
    interpret(table, parse_instructions2(&instructions));
}
