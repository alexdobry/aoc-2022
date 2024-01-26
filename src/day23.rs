use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> [Position; 8] {
        [
            Position::new(self.x - 1, self.y - 1), // NW
            Position::new(self.x, self.y - 1),     // N
            Position::new(self.x + 1, self.y - 1), // NE
            Position::new(self.x + 1, self.y),     // E
            Position::new(self.x + 1, self.y + 1), // SE
            Position::new(self.x, self.y + 1),     // S
            Position::new(self.x - 1, self.y + 1), // SW
            Position::new(self.x - 1, self.y),     // W
        ]
    }

    fn shift(&self, dir: Direction) -> Position {
        match dir {
            N => Position::new(self.x, self.y - 1),
            E => Position::new(self.x + 1, self.y),
            S => Position::new(self.x, self.y + 1),
            W => Position::new(self.x - 1, self.y),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

type World = HashSet<Position>;

fn read_input() -> World {
    let file_path = "inputs/day23.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> World {
    let mut world = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                world.insert(Position {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    world
}

use Direction::*;

pub fn solve1() {
    let mut world = read_input();
    let initial_elf_count = world.len();
    let directions: [Direction; 4] = [N, S, W, E];
    let mut direction_index = 0;

    for _ in 0..10 {
        // Calculate moves
        let mut moves: HashMap<Position, Vec<Position>> = HashMap::new();

        for elf in world.iter() {
            let neighbors = elf.neighbors();
            // Are there no neighbors?
            let mut has_neighbor = false;
            for neighbor in neighbors.iter() {
                if world.contains(neighbor) {
                    has_neighbor = true;
                }
            }
            if !has_neighbor {
                assert!(moves.insert(*elf, vec![*elf]).is_none());
                continue;
            }

            let mut has_stepped = false;
            // Try move rules in order
            for i in 0..4 {
                match directions[(direction_index + i) % 4] {
                    N => {
                        if neighbors[0..3].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(N))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    S => {
                        if neighbors[4..7].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(S))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    E => {
                        if neighbors[2..5].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(E))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    W => {
                        if [neighbors[6], neighbors[7], neighbors[0]]
                            .iter()
                            .all(|n| !world.contains(n))
                        {
                            moves
                                .entry(elf.shift(W))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                }
            }
            if !has_stepped {
                assert!(moves.insert(*elf, vec![*elf]).is_none());
            }
        }

        // Actually step
        world.clear();
        for (pos, previous) in moves {
            if previous.len() > 1 {
                for p in previous {
                    assert!(world.insert(p))
                }
            } else {
                assert!(world.insert(pos))
            }
        }

        direction_index = (direction_index + 1) % 4;
    }

    let elf_count = world.len();

    assert_eq!(elf_count, initial_elf_count);

    let mut max_x = i32::min_value();
    let mut min_x = i32::max_value();
    let mut max_y = i32::min_value();
    let mut min_y = i32::max_value();
    for ele in world {
        max_x = max_x.max(ele.x);
        min_x = min_x.min(ele.x);

        max_y = max_y.max(ele.y);
        min_y = min_y.min(ele.y);
    }

    let result = (max_x - min_x + 1) * (max_y - min_y + 1) - elf_count as i32;
    println!("{result}")
}

pub fn solve2() {
    let mut world = read_input();
    let initial_elf_count = world.len();
    let directions: [Direction; 4] = [N, S, W, E];
    let mut direction_index = 0;

    let mut rounds = 0;
    loop {
        rounds += 1;
        // Calculate moves
        let mut moves: HashMap<Position, Vec<Position>> = HashMap::new();

        for elf in world.iter() {
            let neighbors = elf.neighbors();
            // Are there no neighbors?
            let mut has_neighbor = false;
            for neighbor in neighbors.iter() {
                if world.contains(neighbor) {
                    has_neighbor = true;
                }
            }
            if !has_neighbor {
                assert!(moves.insert(*elf, vec![*elf]).is_none());
                continue;
            }

            let mut has_stepped = false;
            // Try move rules in order
            for i in 0..4 {
                match directions[(direction_index + i) % 4] {
                    N => {
                        if neighbors[0..3].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(N))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    S => {
                        if neighbors[4..7].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(S))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    E => {
                        if neighbors[2..5].iter().all(|n| !world.contains(n)) {
                            moves
                                .entry(elf.shift(E))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                    W => {
                        if [neighbors[6], neighbors[7], neighbors[0]]
                            .iter()
                            .all(|n| !world.contains(n))
                        {
                            moves
                                .entry(elf.shift(W))
                                .and_modify(|positions| positions.push(*elf))
                                .or_insert_with(|| vec![*elf]);
                            has_stepped = true;
                            break;
                        }
                    }
                }
            }
            if !has_stepped {
                assert!(moves.insert(*elf, vec![*elf]).is_none());
            }
        }

        // Actually step
        world.clear();
        let mut has_moved = false;
        for (pos, previous) in moves {
            if previous.len() > 1 {
                has_moved = true;
                for p in previous {
                    assert!(world.insert(p))
                }
            } else {
                if pos != previous[0] {
                    has_moved = true
                }
                assert!(world.insert(pos))
            }
        }
        if !has_moved {
            break;
        }

        direction_index = (direction_index + 1) % 4;
    }

    let elf_count = world.len();

    assert_eq!(elf_count, initial_elf_count);

    println!("{rounds}")
}
