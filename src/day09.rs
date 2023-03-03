use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn parse(str: &str) -> Self {
        match str {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn drag(&self, tail: Position) -> Position {
        let mut tail = tail;
        let dx = self.x - tail.x;
        let dy = self.y - tail.y;
        if dx.abs() > 1 || dy.abs() > 1 {
            tail.x += dx.signum();
            tail.y += dy.signum();
        }
        tail
    }
}

fn read_input() -> Vec<Direction> {
    let file_path = "inputs/day09.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Direction> {
    let mut result = vec![];
    for line in input.lines() {
        let (direction, count) = line.split_once(' ').unwrap();
        let count: i32 = count.parse().unwrap();
        let direction = Direction::parse(direction);
        for _ in 0..count {
            result.push(direction)
        }
    }
    result
}

pub fn solve1() {
    let directions = read_input();
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut tail_visited = HashSet::new();
    tail_visited.insert(tail.clone());

    for direction in directions {
        head.step(direction);
        tail = head.drag(tail.clone());
        tail_visited.insert(tail.clone());
    }

    let result = tail_visited.len();
    println!("{result}");
}

pub fn solve2() {
    let directions = read_input();
    let mut positions = Vec::from_iter((0..10).map(|_| Position::default()));
    let mut tail_visited = HashSet::new();
    tail_visited.insert(positions[9].clone());

    for direction in directions {
        positions[0].step(direction);
        for i in 0..9 {
            positions[i + 1] = positions[i].drag(positions[i + 1].clone());
        }
        tail_visited.insert(positions[9].clone());
    }

    let result = tail_visited.len();
    println!("{result}");
}
