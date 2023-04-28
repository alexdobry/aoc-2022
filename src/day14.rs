use std::collections::HashSet;
use std::fs;

type Coord = (i32, i32);

fn read_input() -> (HashSet<Coord>, i32) {
    let file_path = "inputs/day14.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_coord(str: &str) -> (i32, i32) {
    let (left, right) = str.split_once(',').unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

fn parse_input(input: String) -> (HashSet<Coord>, i32) {
    let mut coords = HashSet::new();
    let mut abyss = 0;

    for path in input.lines() {
        let (first, rest) = path.split_once(" -> ").unwrap();
        let mut current = parse_coord(first);
        coords.insert(current);

        for next in rest.split(" -> ").map(parse_coord) {
            if current.0 == next.0 {
                for y in current.1.min(next.1)..=current.1.max(next.1) {
                    abyss = abyss.max(y);
                    coords.insert((current.0, y));
                }
            } else {
                for x in current.0.min(next.0)..=current.0.max(next.0) {
                    coords.insert((x, current.1));
                }
            }
            current = next;
        }
    }
    (coords, abyss)
}

pub fn solve1() {
    let (mut world, abyss) = read_input();
    let mut result = 0;

    'outer: loop {
        let mut sand = (500, 0);
        loop {
            if sand.1 >= abyss {
                break 'outer;
            }
            if !world.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !world.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 -= 1;
            } else if !world.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                world.insert(sand);
                break;
            }
        }
        result += 1;
    }

    println!("{result}");
}

pub fn solve2() {
    let (mut world, abyss) = read_input();
    let floor = abyss + 2;
    let mut result = 0;

    loop {
        let mut sand = (500, 0);
        if world.contains(&sand) {
            break;
        }
        loop {
            if sand.1 == floor - 1 {
                world.insert(sand);
                break;
            } else if !world.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
            } else if !world.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 -= 1;
            } else if !world.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.1 += 1;
                sand.0 += 1;
            } else {
                world.insert(sand);
                break;
            }
        }
        result += 1;
    }

    println!("{result}");
}
