use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::mem;
use Direction::*;

const X: usize = 100;
const Y: usize = 35;
const T: usize = X * Y;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            N => '^',
            S => 'v',
            W => '<',
            E => '>',
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position {
            x: x.rem_euclid(X),
            y: y.rem_euclid(Y),
        }
    }

    fn shift(&self, dir: Direction) -> Position {
        match dir {
            N => Position::new(self.x, self.y + Y - 1),
            E => Position::new(self.x + 1, self.y),
            S => Position::new(self.x, self.y + 1),
            W => Position::new(self.x + X - 1, self.y),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field(Vec<Direction>);

impl Field {
    fn add(&mut self, dir: Direction) {
        self.0.push(dir)
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct World(Vec<Field>);

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "#".repeat(X + 1))?;
        for (index, field) in self.0.iter().enumerate() {
            if index % X == 0 {
                write!(f, "#\n#")?;
            }
            match field.0.len() {
                0 => write!(f, ".")?,
                1 => write!(f, "{}", field.0[0])?,
                x => write!(f, "{x}")?,
            }
        }
        write!(f, "#\n{}", "#".repeat(X + 2))?;
        Ok(())
    }
}

impl World {
    fn new() -> World {
        let mut fields = Vec::with_capacity(X * Y);
        for _ in 0..(X * Y) {
            fields.push(Field(vec![]))
        }

        World(fields)
    }

    fn get(&self, position: Position) -> &Field {
        self.0.get(position.y * X + position.x).unwrap()
    }

    fn get_mut(&mut self, position: Position) -> &mut Field {
        self.0.get_mut(position.y * X + position.x).unwrap()
    }

    fn add(&mut self, position: Position, dir: Direction) {
        self.get_mut(position).add(dir)
    }

    fn clear(&mut self) {
        for ele in self.0.iter_mut() {
            ele.0.clear()
        }
    }

    fn evolve(&self, next_world: &mut World) {
        for (index, field) in self.0.iter().enumerate() {
            let position = Position::new(index % X, index / X);
            for blizzard in field.0.iter() {
                next_world.add(position.shift(*blizzard), *blizzard)
            }
        }
    }
}

struct BakedWorld(Vec<[[bool; Y]; X]>);

impl BakedWorld {
    fn new() -> BakedWorld {
        let mut result = Vec::with_capacity(T);
        for _ in 0..T {
            result.push([[false; Y]; X])
        }
        BakedWorld(result)
    }

    fn set_occupied(&mut self, pos: Position, t: usize) {
        let board = self.0.get_mut(t % T).unwrap();
        let row = board.get_mut(pos.x).unwrap();
        let col = row.get_mut(pos.y).unwrap();
        *col = true
    }

    fn is_occupied(&self, pos: Position, t: usize) -> bool {
        self.0[t % T][pos.x][pos.y]
    }
}

fn bake_world() -> BakedWorld {
    let mut world = read_input();
    let mut backbuffer = World::new();
    let mut baked_world = BakedWorld::new();

    for t in 0..T {
        for x in 0..X {
            for y in 0..Y {
                let pos = Position { x, y };
                let field = world.get(pos);
                if !field.is_empty() {
                    baked_world.set_occupied(pos, t)
                }
            }
        }

        backbuffer.clear();
        world.evolve(&mut backbuffer);
        mem::swap(&mut world, &mut backbuffer);
    }
    baked_world
}

fn read_input() -> World {
    let file_path = "inputs/day24.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> World {
    let mut fields = vec![];
    for line in input.lines().dropping(1).dropping_back(1) {
        for char in line.chars() {
            if char == '#' {
                continue;
            }
            match char {
                '.' => fields.push(Field(vec![])),
                '<' => fields.push(Field(vec![W])),
                '>' => fields.push(Field(vec![E])),
                '^' => fields.push(Field(vec![N])),
                'v' => fields.push(Field(vec![S])),
                x => panic!("Unknown char {x}"),
            }
        }
    }
    World(fields)
}

pub fn solve1() {
    let baked_world = bake_world();
    let target = Position::new(X - 1, Y - 1);

    let mut visited: HashSet<(usize, Position)> = HashSet::new();
    let mut candidates: BinaryHeap<Reverse<(usize, Position)>> = BinaryHeap::new();
    for t in 1..20 {
        if !baked_world.is_occupied(Position::new(0, 0), t) {
            candidates.push(Reverse((dbg!(t), Position::new(0, 0))));
        }
    }
    while let Some(Reverse((t, pos))) = candidates.pop() {
        if !visited.insert((t, pos)) {
            continue;
        }
        dbg!(pos, t, candidates.len());
        if pos == target {
            println!("target found took {} steps", t + 1);
            break;
        }

        if pos.y != Y - 1 && !baked_world.is_occupied(pos.shift(S), t + 1) {
            candidates.push(Reverse((t + 1, pos.shift(S))))
        }
        if pos.x != X - 1 && !baked_world.is_occupied(pos.shift(E), t + 1) {
            candidates.push(Reverse((t + 1, pos.shift(E))))
        }
        if pos.y != 0 && !baked_world.is_occupied(pos.shift(N), t + 1) {
            candidates.push(Reverse((t + 1, pos.shift(N))))
        }
        if pos.x != 0 && !baked_world.is_occupied(pos.shift(W), t + 1) {
            candidates.push(Reverse((t + 1, pos.shift(W))))
        }
        if !baked_world.is_occupied(pos, t + 1) {
            candidates.push(Reverse((t + 1, pos)))
        }
    }
    println!("end loop")
}

pub fn solve2() {
    let baked_world = bake_world();
    let mut targets = vec![
        Position::new(X - 1, Y - 1),
        Position::new(0, 0),
        Position::new(X - 1, Y - 1),
    ];

    let mut visited: HashSet<(usize, Position)> = HashSet::new();
    let mut candidates: BinaryHeap<Reverse<(usize, Position)>> = BinaryHeap::new();
    for t in 1..20 {
        if !baked_world.is_occupied(Position::new(0, 0), t) {
            candidates.push(Reverse((dbg!(t), Position::new(0, 0))));
        }
    }
    while let Some(target) = targets.pop() {
        while let Some(Reverse((t, pos))) = candidates.pop() {
            if !visited.insert((t, pos)) {
                continue;
            }
            if pos == target {
                println!("target {target:?} found, at time {}", t + 1);
                candidates.clear();

                for t in t + 2..t + 22 {
                    if !baked_world.is_occupied(target, t) {
                        candidates.push(Reverse((t, target)));
                    }
                }
                break;
            }

            if pos.y != Y - 1 && !baked_world.is_occupied(pos.shift(S), t + 1) {
                candidates.push(Reverse((t + 1, pos.shift(S))))
            }
            if pos.x != X - 1 && !baked_world.is_occupied(pos.shift(E), t + 1) {
                candidates.push(Reverse((t + 1, pos.shift(E))))
            }
            if pos.y != 0 && !baked_world.is_occupied(pos.shift(N), t + 1) {
                candidates.push(Reverse((t + 1, pos.shift(N))))
            }
            if pos.x != 0 && !baked_world.is_occupied(pos.shift(W), t + 1) {
                candidates.push(Reverse((t + 1, pos.shift(W))))
            }
            if !baked_world.is_occupied(pos, t + 1) {
                candidates.push(Reverse((t + 1, pos)))
            }
        }
    }
    println!("end")
}
