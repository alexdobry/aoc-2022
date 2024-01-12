use std::collections::HashMap;
use std::hash::Hash;
use std::{fs, vec};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub(crate) fn shift(&self, x: i32, y: i32) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

type TraversalMap = HashMap<(Pos, Direction), (Pos, Direction)>;

#[derive(Debug)]
struct World {
    world: Vec<Vec<char>>,
}

impl World {
    fn get(&self, pos: Pos) -> char {
        self.world[(pos.y - 1) as usize][(pos.x - 1) as usize]
    }

    pub fn find_leftmost(&self, line: i32) -> Pos {
        Pos {
            x: (self.world[(line - 1) as usize]
                .iter()
                .position(|c| *c != ' ')
                .unwrap()
                + 1) as i32,
            y: line,
        }
    }

    pub fn find_rightmost(&self, line: i32) -> Pos {
        Pos {
            x: (self.world[(line - 1) as usize]
                .iter()
                .rposition(|c| *c != ' ')
                .unwrap()
                + 1) as i32,
            y: line,
        }
    }

    pub fn find_topmost(&self, column: i32) -> Pos {
        Pos {
            x: column,
            y: (self
                .world
                .iter()
                .position(|l| l.get(column as usize - 1).unwrap_or(&' ') != &' ')
                .unwrap()
                + 1) as i32,
        }
    }

    pub fn find_botmost(&self, column: i32) -> Pos {
        Pos {
            x: column,
            y: (self
                .world
                .iter()
                .rposition(|l| l.get(column as usize - 1).unwrap_or(&' ') != &' ')
                .unwrap()
                + 1) as i32,
        }
    }

    pub fn step_cube(
        &self,
        traversal_map: &TraversalMap,
        current: Pos,
        direction: Direction,
    ) -> (Pos, Direction) {
        let new_pos = match direction {
            Direction::Left => current.shift(-1, 0),
            Direction::Right => current.shift(1, 0),
            Direction::Top => current.shift(0, -1),
            Direction::Bot => current.shift(0, 1),
        };
        let (new_pos, new_direction) = traversal_map
            .get(&(new_pos, direction))
            .copied()
            .unwrap_or((new_pos, direction));
        let char = self.get(new_pos);
        assert_ne!(current, new_pos);
        match char {
            '#' => (current, direction),
            '.' => (new_pos, new_direction),
            _ => panic!(),
        }
    }

    pub fn step(&self, current: Pos, direction: Direction) -> Pos {
        match direction {
            Direction::Left => {
                let new_pos = current.shift(-1, 0);
                let char = self.get(new_pos);
                match char {
                    '#' => current,
                    '.' => new_pos,
                    _ => {
                        let new_pos = self.find_rightmost(current.y);
                        let char = self.get(new_pos);
                        match char {
                            '#' => current,
                            '.' => new_pos,
                            _ => panic!(),
                        }
                    }
                }
            }
            Direction::Right => {
                let new_pos = current.shift(1, 0);
                let char = self.get(new_pos);
                match char {
                    '#' => current,
                    '.' => new_pos,
                    _ => {
                        let new_pos = self.find_leftmost(current.y);
                        let char = self.get(new_pos);
                        match char {
                            '#' => current,
                            '.' => new_pos,
                            _ => panic!(),
                        }
                    }
                }
            }
            Direction::Top => {
                let new_pos = current.shift(0, -1);
                let char = self.get(new_pos);
                match char {
                    '#' => current,
                    '.' => new_pos,
                    _ => {
                        let new_pos = self.find_botmost(current.x);
                        let char = self.get(new_pos);
                        match char {
                            '#' => current,
                            '.' => new_pos,
                            _ => panic!(),
                        }
                    }
                }
            }
            Direction::Bot => {
                let new_pos = current.shift(0, 1);
                let char = self.get(new_pos);
                match char {
                    '#' => current,
                    '.' => new_pos,
                    _ => {
                        let new_pos = self.find_topmost(current.x);
                        let char = self.get(new_pos);
                        match char {
                            '#' => current,
                            '.' => new_pos,
                            _ => panic!(),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    Go(i32),
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Top,
    Bot,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Left => Direction::Bot,
            Direction::Right => Direction::Top,
            Direction::Top => Direction::Left,
            Direction::Bot => Direction::Right,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Left => Direction::Top,
            Direction::Right => Direction::Bot,
            Direction::Top => Direction::Right,
            Direction::Bot => Direction::Left,
        }
    }
}

fn read_input() -> (World, Vec<Instr>) {
    let file_path = "inputs/day22.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

#[allow(unstable_name_collisions)]
fn parse_input(input: String) -> (World, Vec<Instr>) {
    let mut lines = input.lines();
    let mut world = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        world.push(line.chars().collect());
    }

    let instrs = lines
        .next()
        .unwrap()
        .split('L')
        .map(|rs| {
            rs.split('R')
                .map(|d| Instr::Go(d.parse().unwrap()))
                .intersperse(Instr::Right)
                .collect::<Vec<_>>()
        })
        .intersperse(vec![Instr::Left])
        .flatten()
        .collect();

    (World { world }, instrs)
}

pub fn solve1() {
    let (world, instrs) = read_input();
    let mut current = world.find_leftmost(1);
    let mut direction = Direction::Right;
    for instr in instrs {
        match instr {
            Instr::Go(steps) => {
                for _ in 0..steps {
                    current = world.step(current, direction);
                }
            }
            Instr::Left => direction = direction.turn_left(),
            Instr::Right => direction = direction.turn_right(),
        }
    }
    let facing = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Top => 3,
        Direction::Bot => 1,
    };
    let result = 1000 * current.y + 4 * current.x + facing;
    println!("{result}");
}

fn range(from: Pos, to: Pos) -> Vec<Pos> {
    let res: Vec<Pos> = if from.x == to.x {
        if from.y <= to.y {
            (from.y..=to.y).map(|y| Pos { x: from.x, y }).collect()
        } else {
            (to.y..=from.y).map(|y| Pos { x: from.x, y }).collect()
        }
    } else {
        assert_eq!(from.y, to.y);
        if from.x <= to.x {
            (from.x..=to.x).map(|x| Pos { x, y: from.y }).collect()
        } else {
            (to.x..=from.x).map(|x| Pos { x, y: from.y }).collect()
        }
    };
    assert_eq!(res.len(), 50);
    res
}

fn add_edge(
    map: &mut TraversalMap,
    from: Vec<Pos>,
    to: Vec<Pos>,
    old_dir: Direction,
    new_dir: Direction,
) {
    assert_eq!(from.len(), to.len());
    for (from, to) in from.into_iter().zip(to.into_iter()) {
        assert!(map.insert((from, old_dir), (to, new_dir)).is_none());
    }
}

pub fn solve2() {
    let mut traversal_map: TraversalMap = HashMap::new();
    add_edge(
        &mut traversal_map,
        range(Pos { x: 51, y: 0 }, Pos { x: 100, y: 0 }),
        range(Pos { x: 1, y: 151 }, Pos { x: 1, y: 200 }),
        Direction::Top,
        Direction::Right,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 101, y: 0 }, Pos { x: 150, y: 0 }),
        range(Pos { x: 1, y: 200 }, Pos { x: 50, y: 200 }),
        Direction::Top,
        Direction::Top,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 151, y: 1 }, Pos { x: 151, y: 50 }),
        range(Pos { x: 100, y: 150 }, Pos { x: 100, y: 101 }),
        Direction::Right,
        Direction::Left,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 101, y: 51 }, Pos { x: 150, y: 51 }),
        range(Pos { x: 100, y: 51 }, Pos { x: 100, y: 100 }),
        Direction::Bot,
        Direction::Left,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 101, y: 51 }, Pos { x: 101, y: 100 }),
        range(Pos { x: 101, y: 50 }, Pos { x: 150, y: 50 }),
        Direction::Right,
        Direction::Top,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 101, y: 101 }, Pos { x: 101, y: 150 }),
        range(Pos { x: 150, y: 50 }, Pos { x: 150, y: 1 }),
        Direction::Right,
        Direction::Left,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 51, y: 151 }, Pos { x: 100, y: 151 }),
        range(Pos { x: 50, y: 151 }, Pos { x: 50, y: 200 }),
        Direction::Bot,
        Direction::Left,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 51, y: 151 }, Pos { x: 51, y: 200 }),
        range(Pos { x: 51, y: 150 }, Pos { x: 100, y: 150 }),
        Direction::Right,
        Direction::Top,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 1, y: 201 }, Pos { x: 50, y: 201 }),
        range(Pos { x: 101, y: 1 }, Pos { x: 150, y: 1 }),
        Direction::Bot,
        Direction::Bot,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 0, y: 151 }, Pos { x: 0, y: 200 }),
        range(Pos { x: 51, y: 1 }, Pos { x: 100, y: 1 }),
        Direction::Left,
        Direction::Bot,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 0, y: 101 }, Pos { x: 0, y: 150 }),
        range(Pos { x: 51, y: 50 }, Pos { x: 51, y: 1 }),
        Direction::Left,
        Direction::Right,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 1, y: 100 }, Pos { x: 50, y: 100 }),
        range(Pos { x: 51, y: 51 }, Pos { x: 51, y: 100 }),
        Direction::Top,
        Direction::Right,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 50, y: 51 }, Pos { x: 50, y: 100 }),
        range(Pos { x: 1, y: 101 }, Pos { x: 50, y: 101 }),
        Direction::Left,
        Direction::Bot,
    );
    add_edge(
        &mut traversal_map,
        range(Pos { x: 50, y: 1 }, Pos { x: 50, y: 50 }),
        range(Pos { x: 1, y: 150 }, Pos { x: 1, y: 101 }),
        Direction::Left,
        Direction::Right,
    );
    dbg!(traversal_map.len() / 14);

    for ((start_pos, start_dir), (target_pos, target_direction)) in traversal_map.clone() {
        let new_dir = target_direction.turn_left().turn_left();
        let new_pos = match new_dir {
            Direction::Left => target_pos.shift(-1, 0),
            Direction::Right => target_pos.shift(1, 0),
            Direction::Top => target_pos.shift(0, -1),
            Direction::Bot => target_pos.shift(0, 1),
        };

        let new_start_pos = match start_dir.turn_left().turn_left() {
            Direction::Left => start_pos.shift(-1, 0),
            Direction::Right => start_pos.shift(1, 0),
            Direction::Top => start_pos.shift(0, -1),
            Direction::Bot => start_pos.shift(0, 1),
        };
        assert_eq!(
            traversal_map.get(&(new_pos, new_dir)).unwrap(),
            &(new_start_pos, start_dir.turn_left().turn_left())
        );
    }

    let (world, instrs) = read_input();

    let mut current = dbg!(world.find_leftmost(1));
    let mut direction = Direction::Right;
    for instr in instrs {
        match instr {
            Instr::Go(steps) => {
                for _ in 0..steps {
                    let (new_current, new_direction) =
                        world.step_cube(&traversal_map, current, direction);
                    current = new_current;
                    direction = new_direction;
                }
            }
            Instr::Left => direction = direction.turn_left(),
            Instr::Right => direction = direction.turn_right(),
        }
    }
    let facing = match direction {
        Direction::Left => 2,
        Direction::Right => 0,
        Direction::Top => 3,
        Direction::Bot => 1,
    };
    let result = 1000 * current.y + 4 * current.x + facing;
    println!("{result}");
}
