use std::collections::HashSet;
use std::fs;

fn read_input() -> Vec<String> {
    let file_path = "inputs/day03.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents.lines().map(|l| l.to_owned()).collect()
}

fn priority(c: char) -> u32 {
    let code: u32 = if c.is_ascii_uppercase() { 38 } else { 96 };
    c as u32 - code
}

fn find_duplicate<'a, T>(mut strs: T) -> char
where
    T: Iterator<Item = &'a str>,
{
    let initial: HashSet<char> = strs.next().unwrap().chars().collect();
    let common_chars = strs.fold(initial, |acc, s| {
        let other: HashSet<char> = s.chars().collect();
        let intersection = acc.intersection(&other);
        intersection.map(|c| c.to_owned()).collect()
    });
    common_chars.iter().next().unwrap().to_owned()
}

pub fn solve1() {
    let inputs = read_input();
    let result: u32 = inputs
        .into_iter()
        .map(|line| {
            let split = line.split_at(line.len() / 2);
            priority(find_duplicate(vec![split.0, split.1].into_iter()))
        })
        .sum();
    println!("{result}")
}

pub fn solve2() {
    let inputs = read_input();
    let result: u32 = inputs
        .chunks(3)
        .map(|chunk| priority(find_duplicate(chunk.iter().map(|c| c.as_str()))))
        .sum();
    println!("{result}")
}
