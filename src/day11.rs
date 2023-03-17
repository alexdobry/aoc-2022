use nom::{
    branch::alt, bytes::complete::tag, character::complete as text, combinator::map, multi,
    sequence::tuple, IResult,
};
use std::fs;

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<i32>,
    test: (i32, usize, usize),
    operation: (Op, Option<i32>),
}

impl Monkey {
    fn inspect(&self, worry_level: i32) -> i32 {
        let operand = self.operation.1.unwrap_or(worry_level);
        match self.operation.0 {
            Op::Add => worry_level + operand,
            Op::Multiply => worry_level * operand,
        }
    }
}

fn read_input() -> Vec<Monkey> {
    let file_path = "inputs/day11.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = tuple((text::multispace1, tag("Starting items: ")))(input)?;
    multi::separated_list0(tag(", "), text::i32)(input)
}

fn parse_operation(input: &str) -> IResult<&str, (Op, Option<i32>)> {
    let (input, _) = tuple((text::multispace1, tag("Operation: new = old ")))(input)?;
    let (input, operation) =
        alt((map(tag("+"), |_| Op::Add), map(tag("*"), |_| Op::Multiply)))(input)?;
    let (input, _) = text::multispace1(input)?;
    let (input, operand) = alt((map(tag("old"), |_| None), map(text::i32, |i| Some(i))))(input)?;
    Ok((input, (operation, operand)))
}

fn parse_test(input: &str) -> IResult<&str, (i32, usize, usize)> {
    let (input, _) = tuple((text::multispace1, tag("Test: divisible by ")))(input)?;
    let (input, div) = text::i32(input)?;
    let (input, _) = text::newline(input)?;
    let (input, _) = tuple((text::multispace1, tag("If true: throw to monkey ")))(input)?;
    let (input, then_monkey) = text::u32(input)?;
    let (input, _) = text::newline(input)?;
    let (input, _) = tuple((text::multispace1, tag("If false: throw to monkey ")))(input)?;
    let (input, else_monkey) = text::u32(input)?;
    Ok((input, (div, then_monkey as usize, else_monkey as usize)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tuple((
        tag("Monkey"),
        text::multispace1,
        text::i32,
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
    let monkeys = read_input();
    println!("{monkeys:#?}");
}
