use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub fn m_dist(&self, other: &Coord) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn shift_x(&self, x: i64) -> Coord {
        Coord {
            x: self.x + x,
            y: self.y,
        }
    }

    pub fn shift_y(&self, y: i64) -> Coord {
        Coord {
            x: self.x,
            y: self.y + y,
        }
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
        let x_min = x1 - x_dist as i64;
        let x_max = x1 + x_dist as i64;
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

struct Rect {
    point: Coord,
    len_x: u64,
    len_y: u64,
}

impl Rect {
    fn covered_by(&self, sensor: &Coord, beacon: &Coord) -> bool {
        let dist = sensor.m_dist(beacon);
        let d1 = sensor.m_dist(&self.point);
        let d2 = sensor.m_dist(&self.point.shift_x(self.len_x as i64));
        let d3 = sensor.m_dist(&self.point.shift_y(self.len_y as i64));
        let d4 = sensor.m_dist(
            &self
                .point
                .shift_y(self.len_y as i64)
                .shift_x(self.len_x as i64),
        );
        d1 <= dist && d2 <= dist && d3 <= dist && d4 <= dist
    }
}

pub fn solve2() {
    let coords = read_input();
    let mut stack = vec![];
    stack.push(Rect {
        point: Coord { x: 0, y: 0 },
        len_x: 4000000,
        len_y: 4000000,
    });
    'outer: while let Some(current) = stack.pop() {
        for (sensor, beacon) in coords.iter() {
            if current.covered_by(sensor, beacon) {
                continue 'outer;
            }
        }
        let half_x = current.len_x / 2;
        let half_y = current.len_y / 2;

        if current.len_x == 0 && current.len_y == 0 {
            println!("{:?}", current.point.x * 4000000 + current.point.y);
            break;
        }

        stack.push(Rect {
            point: current.point.clone(),
            len_x: half_x,
            len_y: half_y,
        });

        if half_x != 0 {
            stack.push(Rect {
                point: current.point.shift_x(half_x as i64),
                len_x: current.len_x - half_x,
                len_y: half_y,
            });
        }

        if half_y != 0 {
            stack.push(Rect {
                point: current.point.shift_y(half_y as i64),
                len_x: half_x,
                len_y: current.len_y - half_y,
            });
        }

        if half_x != 0 && half_y != 0 {
            stack.push(Rect {
                point: current.point.shift_y(half_y as i64).shift_x(half_x as i64),
                len_x: current.len_x - half_x,
                len_y: current.len_y - half_y,
            });
        }
    }
}
