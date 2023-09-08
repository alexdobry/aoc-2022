use std::collections::HashSet;
use std::fs;
use std::ops::Div;

use nom::AsChar;

fn read_input() -> String {
    let file_path = "inputs/day17.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> String {
    input.trim().to_string()
}

type Coord = (i64, i64);

struct World {
    stones: HashSet<Coord>,
    top: i64,
}

impl World {
    pub(crate) fn highest(&self) -> i64 {
        self.top
    }

    pub(crate) fn insert(&mut self, coord: Coord) {
        self.top = self.top.max(coord.1);
        self.stones.insert(coord);
    }

    pub(crate) fn new(y: i64) -> Self {
        World {
            stones: (0..7).map(|x| (x, y)).collect(),
            top: y,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn draw(&self) {
        for y in (0..=self.top).rev() {
            println!();
            for x in 0..7 {
                print!(
                    "{}",
                    if self.stones.contains(&(x, y)) {
                        "#"
                    } else {
                        "."
                    }
                )
            }
        }
        println!();
        println!()
    }

    pub(crate) fn levels(&self) -> [i64; 7] {
        let mut lvls = [0; 7];
        for (x, y) in self.stones.iter() {
            lvls[*x as usize] = lvls[*x as usize].max(*y);
        }
        lvls
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Shape {
    // +###
    Flat,
    // .#.
    // ###
    // +#.
    Plus,
    // ..#
    // ..#
    // +##
    ReverseL,
    // #
    // #
    // #
    // +
    Bar,
    // ##
    // +#
    Square,
}

impl Shape {
    pub(crate) fn fall(&self, coord: &mut Coord, world: &mut World) -> bool {
        let new_coord = (coord.0, coord.1 - 1);
        let coords: Vec<Coord> = self.occupies(&new_coord);
        let collides = coords
            .into_iter()
            .any(|coord| world.stones.contains(&coord));
        if collides {
            self.occupies(coord)
                .into_iter()
                .for_each(|coord| world.insert(coord));
            true
        } else {
            *coord = new_coord;
            false
        }
    }

    fn occupies(&self, coord: &Coord) -> Vec<Coord> {
        let (x, y) = *coord;
        match self {
            Shape::Flat => {
                vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]
            }
            Shape::Plus => {
                vec![
                    (x + 1, y),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 2, y + 1),
                    (x + 1, y + 2),
                ]
            }
            Shape::ReverseL => {
                vec![
                    (x, y),
                    (x + 1, y),
                    (x + 2, y),
                    (x + 2, y + 1),
                    (x + 2, y + 2),
                ]
            }
            Shape::Bar => {
                vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]
            }
            Shape::Square => {
                vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
            }
        }
    }
}

impl Shape {
    #[allow(clippy::manual_range_contains)]
    pub(crate) fn jet(&self, jet: i64, coord: &mut Coord, world: &World) {
        let new_coord = (coord.0 + jet, coord.1);
        let coords: Vec<Coord> = self.occupies(&new_coord);
        let collides = coords
            .into_iter()
            .any(|(x, y)| x < 0 || x > 6 || world.stones.contains(&(x, y)));
        if !collides {
            *coord = new_coord;
        }
    }
}

fn jet_at(jets: &str, idx: &mut usize) -> i64 {
    let bytes = jets.as_bytes();
    let res = if bytes[*idx].as_char() == '<' { -1 } else { 1 };
    *idx += 1;
    if *idx == bytes.len() {
        *idx = 0
    }
    res
}

pub fn solve1() {
    let jets = read_input();
    let mut jet_index: usize = 0;
    let shapes = vec![
        Shape::Flat,
        Shape::Plus,
        Shape::ReverseL,
        Shape::Bar,
        Shape::Square,
    ];
    let mut world = World::new(0);
    let rocks = 2022;

    for shape in shapes.iter().cycle().take(rocks) {
        let mut coord = (2, world.highest() + 4);
        loop {
            let jet = jet_at(&jets, &mut jet_index);
            shape.jet(jet, &mut coord, &world);
            if shape.fall(&mut coord, &mut world) {
                break;
            }
        }
    }

    let result = world.highest();
    println!("{result}");
}

pub fn solve2() {
    let jets = read_input();
    let mut jet_index: usize = 0;
    let shapes = vec![
        Shape::Flat,
        Shape::Plus,
        Shape::ReverseL,
        Shape::Bar,
        Shape::Square,
    ];
    let mut world = World::new(0);
    let mut shape_count = 0;
    let y_diff = 2711;
    let shape_diff = 1735;
    let rocks: i64 = 1_000_000_000_000;
    let mut iter_shape = &shapes[0];
    let iter_jet = 9700;

    for shape in shapes.iter().cycle() {
        let is_flat = world.levels().into_iter().all(|l| l == world.highest());
        if is_flat && jet_index == iter_jet {
            iter_shape = shape;
            break;
        }
        shape_count += 1;
        let mut coord = (2, world.highest() + 4);
        loop {
            let jet = jet_at(&jets, &mut jet_index);
            shape.jet(jet, &mut coord, &world);
            if shape.fall(&mut coord, &mut world) {
                break;
            }
        }
    }

    let iter_count = (rocks - shape_count).div(shape_diff);
    let new_highest = world.highest() + y_diff * iter_count;
    world = World::new(new_highest);

    let remaining_shapes = rocks - shape_count - shape_diff * iter_count;
    for shape in shapes
        .iter()
        .cycle()
        .skip_while(|s| *s != iter_shape)
        .take(remaining_shapes as usize)
    {
        shape_count += 1;
        let mut coord = (2, world.highest() + 4);
        loop {
            let jet = jet_at(&jets, &mut jet_index);
            shape.jet(jet, &mut coord, &world);
            if shape.fall(&mut coord, &mut world) {
                break;
            }
        }
    }

    let result = world.highest();
    println!("{result}");
}
