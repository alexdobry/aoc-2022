use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    Add(i32),
}

fn read_input() -> Vec<Instruction> {
    let file_path = "inputs/day10.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Instruction> {
    let mut result = vec![];
    for line in input.lines() {
        if line == "noop" {
            result.push(Instruction::Noop);
        } else {
            let (_, count) = line.split_once(' ').unwrap();
            let count: i32 = count.parse().unwrap();
            result.push(Instruction::Noop);
            result.push(Instruction::Add(count));
        }
    }
    result
}

pub fn solve1() {
    let instructions = read_input();
    let mut reg_x: i32 = 1;
    let mut result: i32 = 0;
    for (i, instr) in instructions.into_iter().enumerate() {
        let i = i + 1;
        if (i + 20) % 40 == 0 {
            result += i as i32 * reg_x;
        }
        match instr {
            Instruction::Noop => {}
            Instruction::Add(v) => {
                reg_x += v;
            }
        }
    }
    println!("{result}");
}

pub fn solve2() {
    let instructions = read_input();
    let mut reg_x: i32 = 1;
    for (i, instr) in instructions.into_iter().enumerate() {
        let electron_x = i as i32 % 40;
        if electron_x == 0 {
            println!()
        }
        if (electron_x - reg_x).abs() < 2 {
            print!("#")
        } else {
            print!(".")
        }
        match instr {
            Instruction::Noop => {}
            Instruction::Add(v) => {
                reg_x += v;
            }
        }
    }
}
