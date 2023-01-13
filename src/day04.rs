use std::fs;

type ElfRange = (u32, u32);

fn parse_elf_range(str: &str) -> ElfRange {
    let (l, r) = str.split_once("-").unwrap();
    (l.parse().unwrap(), r.parse().unwrap())
}

fn read_input() -> Vec<(ElfRange, ElfRange)> {
    let file_path = "inputs/day04.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(",").unwrap();
            (parse_elf_range(l), parse_elf_range(r))
        })
        .collect()
}

fn includes(l: ElfRange, r: ElfRange) -> bool {
    l.0 <= r.0 && l.1 >= r.1
}

fn overlaps(l: ElfRange, r: ElfRange) -> bool {
    l.0 <= r.0 && l.1 >= r.0 || r.0 <= l.0 && r.1 >= l.0
}

pub fn solve1() {
    let inputs = read_input();
    let mut result = 0;
    for (l, r) in inputs {
        if includes(l, r) || includes(r, l) {
            result += 1;
        }
    }
    println!("{result}")
}

pub fn solve2() {
    let inputs = read_input();
    let mut result = 0;
    for (l, r) in inputs {
        if overlaps(l, r) {
            result += 1;
        }
    }
    println!("{result}")
}
