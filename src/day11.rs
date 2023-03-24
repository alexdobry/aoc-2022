use nom::{
    branch::alt, bytes::complete::tag, character::complete as text, combinator::map, multi,
    sequence::tuple, IResult,
};
use std::fs;
use std::mem;

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<u64>,
    test: (u64, usize, usize),
    operation: (Op, Option<u64>),
    inspected_items: u64,
}

impl Monkey {
    fn inspect(&mut self, worry_level: u64) -> u64 {
        self.inspected_items += 1;
        let ring = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
        let operand = self.operation.1.unwrap_or(worry_level);
        let worry_level = worry_level % ring;
        let operand = operand % ring;
        match self.operation.0 {
            Op::Add => worry_level + operand,
            Op::Multiply => worry_level * operand,
        }
    }

    fn throw(&self, worry_level: u64) -> usize {
        if worry_level % self.test.0 == 0 {
            self.test.1
        } else {
            self.test.2
        }
    }
}

fn read_input() -> Vec<Monkey> {
    let file_path = "inputs/day11.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tuple((text::multispace1, tag("Starting items: ")))(input)?;
    multi::separated_list0(tag(", "), text::u64)(input)
}

fn parse_operation(input: &str) -> IResult<&str, (Op, Option<u64>)> {
    let (input, _) = tuple((text::multispace1, tag("Operation: new = old ")))(input)?;
    let (input, operation) =
        alt((map(tag("+"), |_| Op::Add), map(tag("*"), |_| Op::Multiply)))(input)?;
    let (input, _) = text::multispace1(input)?;
    let (input, operand) = alt((map(tag("old"), |_| None), map(text::u64, Some)))(input)?;
    Ok((input, (operation, operand)))
}

fn parse_test(input: &str) -> IResult<&str, (u64, usize, usize)> {
    let (input, _) = tuple((text::multispace1, tag("Test: divisible by ")))(input)?;
    let (input, div) = text::u64(input)?;
    let (input, _) = text::newline(input)?;
    let (input, _) = tuple((text::multispace1, tag("If true: throw to monkey ")))(input)?;
    let (input, then_monkey) = text::u64(input)?;
    let (input, _) = text::newline(input)?;
    let (input, _) = tuple((text::multispace1, tag("If false: throw to monkey ")))(input)?;
    let (input, else_monkey) = text::u64(input)?;
    Ok((input, (div, then_monkey as usize, else_monkey as usize)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((
        tag("Monkey"),
        text::multispace1,
        text::u64,
        tag(":"),
        text::newline,
    ))(input)?;
    let (input, items) = parse_items(input)?;
    let (input, _) = text::newline(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, _) = text::newline(input)?;
    let (input, test) = parse_test(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            inspected_items: 0,
        },
    ))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    multi::separated_list0(text::multispace1, parse_monkey)(input)
}

fn parse_input(input: String) -> Vec<Monkey> {
    parse_monkeys(&input).unwrap().1
}

pub fn solve1() {
    let mut monkeys = read_input();
    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_index].items);
            for item in items {
                let new_item = monkeys[monkey_index].inspect(item) / 3;
                let next_monkey_index = monkeys[monkey_index].throw(new_item);
                monkeys[next_monkey_index].items.push(new_item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspected_items);
    let result: u64 =
        monkeys[monkeys.len() - 1].inspected_items * monkeys[monkeys.len() - 2].inspected_items;
    println!("{result}");
}

// https://de.wikipedia.org/wiki/Restklassenring
pub fn solve2() {
    let mut monkeys = read_input();
    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[monkey_index].items);
            for item in items {
                let new_item = monkeys[monkey_index].inspect(item);
                let next_monkey_index = monkeys[monkey_index].throw(new_item);
                monkeys[next_monkey_index].items.push(new_item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspected_items);
    let result: u64 =
        monkeys[monkeys.len() - 1].inspected_items * monkeys[monkeys.len() - 2].inspected_items;
    println!("{result}");
}
