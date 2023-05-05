use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn m_dist(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn read_input() -> Vec<(Coord, Coord)> {
    let file_path = "inputs/day15.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<(Coord, Coord)> {
    let mut coords = vec![];
    for line in input.lines() {
        let (_, line) = line.split_once("x=").unwrap();
        let (sensor_x, line) = line.split_once(',').unwrap();
        let (_, line) = line.split_once("y=").unwrap();
        let (sensor_y, line) = line.split_once(':').unwrap();
        let (_, line) = line.split_once("x=").unwrap();
        let (beacon_x, line) = line.split_once(',').unwrap();
        let (_, beacon_y) = line.split_once("y=").unwrap();
        coords.push((
            Coord {
                x: sensor_x.parse().unwrap(),
                y: sensor_y.parse().unwrap(),
            },
            Coord {
                x: beacon_x.parse().unwrap(),
                y: beacon_y.parse().unwrap(),
            },
        ))
    }
    coords
}

pub fn solve1() {
    let coords = read_input();
    let mut blocked_x = HashSet::new();
    for (sensor, beacon) in coords {
        let dist = sensor.m_dist(&beacon);
        let x1 = sensor.x;
        let y1 = sensor.y;
        let y2 = 2000000;
        let y_diff = y1.abs_diff(y2);
        if y_diff > dist {
            continue;
        }
        let x_dist = dist - y_diff;
        let x_min = x1 - x_dist as i32;
        let x_max = x1 + x_dist as i32;
        (x_min..=x_max).for_each(|x| {
            blocked_x.insert(x);
        });
        if beacon.y == 2000000 {
            blocked_x.remove(&beacon.x);
        }
    }
    let result = blocked_x.len();
    println!("{result}");
}
