use std::cmp::Ordering;
use std::fs;

use nom::character::complete::{char, u32};
use nom::{branch::alt, multi, sequence::delimited, IResult, Parser};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tree {
    Leaf(u32),
    Node(Vec<Tree>),
}

fn read_input() -> Vec<(Tree, Tree)> {
    let file_path = "inputs/day13.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn cmp_tree(t1: &Tree, t2: &Tree) -> Ordering {
    match (t1, t2) {
        (Tree::Leaf(l1), Tree::Leaf(l2)) => l1.cmp(l2),
        (Tree::Node(n1), Tree::Node(n2)) => {
            let len_cmp = n1.len().cmp(&n2.len());
            for (l, r) in n1.iter().zip(n2) {
                let cmp = cmp_tree(l, r);
                if !cmp.is_eq() {
                    return cmp;
                }
            }
            len_cmp
        }
        (Tree::Leaf(_), _) => cmp_tree(&Tree::Node(vec![t1.clone()]), t2),
        (_, Tree::Leaf(_)) => cmp_tree(t1, &Tree::Node(vec![t2.clone()])),
    }
}

fn parse_tree(input: &str) -> IResult<&str, Tree> {
    let node = delimited(
        char('['),
        multi::separated_list0(char(','), parse_tree),
        char(']'),
    )
    .map(Tree::Node);
    let leaf = u32.map(Tree::Leaf);
    alt((node, leaf))(input)
}

fn parse_input(input: String) -> Vec<(Tree, Tree)> {
    let mut result = vec![];
    for pair in input.split("\n\n") {
        let (first, second) = pair.split_once('\n').unwrap();
        result.push((parse_tree(first).unwrap().1, parse_tree(second).unwrap().1));
    }
    result
}

pub fn solve1() {
    let trees = read_input();
    let mut result = 0;
    for (i, (left, right)) in trees.iter().enumerate() {
        if cmp_tree(left, right).is_lt() {
            result += i + 1;
        }
    }
    println!("{result}");
}

pub fn solve2() {
    let trees = read_input();
    let marker1 = parse_tree("[[2]]").unwrap().1;
    let marker2 = parse_tree("[[6]]").unwrap().1;
    let mut trees: Vec<Tree> = trees.into_iter().flat_map(|(l, r)| vec![l, r]).collect();
    trees.push(marker1.clone());
    trees.push(marker2.clone());
    trees.sort_by(cmp_tree);

    let pos1 = trees.iter().position(|t| t == &marker1).unwrap() + 1;
    let pos2 = trees.iter().position(|t| t == &marker2).unwrap() + 1;
    let result = pos1 * pos2;
    println!("{result}");
}
