use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::Hash;

#[derive(Clone, Debug)]
struct Room {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
    shortest_paths: Vec<(String, u32)>,
}

#[derive(Debug)]
enum Step {
    Open,
    MoveTo(String),
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input() -> Vec<Room> {
    let file_path = "inputs/day16.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Room> {
    let mut rooms = vec![];
    for line in input.lines() {
        let (_, line) = line.split_once("Valve ").unwrap();
        let (name, line) = line.split_once(" has flow rate=").unwrap();
        let (flow_rate, line) = line
            .split_once("; tunnels lead to valves ")
            .or_else(|| line.split_once("; tunnel leads to valve "))
            .unwrap();
        let tunnels = line.split(", ");
        rooms.push(Room {
            name: name.to_string(),
            flow_rate: flow_rate.parse().unwrap(),
            tunnels: tunnels.map(|t| t.to_string()).collect(),
            shortest_paths: vec![],
        })
    }
    rooms
}

fn eval(rooms: &Vec<Room>, mut steps: Vec<String>) -> u32 {
    steps.reverse();
    let mut released_pressure: u32 = 0;
    let mut open_valves = HashSet::new();
    let mut current_room = "AA".to_string();
    let mut minute_counter = 0;

    while minute_counter < 30 {
        let open_pressure = rooms
            .iter()
            .map(|r| {
                if open_valves.contains(r.name.as_str()) {
                    r.flow_rate
                } else {
                    0
                }
            })
            .sum::<u32>();

        match steps.pop() {
            None => {
                released_pressure += (30 - minute_counter) * open_pressure;
                break;
            }
            Some(step) => {
                let room = rooms.iter().find(|r| r.name == current_room).unwrap();
                let distance = match room.shortest_paths.iter().find(|ps| ps.0 == step) {
                    None => {
                        // room == AA == step
                        // dbg!(room);
                        // dbg!(step);
                        // dbg!(steps);
                        0
                    }
                    Some(res) => res.1,
                };
                minute_counter += distance + 1;
                assert!(minute_counter < 30);
                released_pressure += (distance + 1) * open_pressure;

                current_room = step;
                if !open_valves.insert(current_room.clone()) {
                    panic!("double open {current_room}")
                }
            }
        }
    }
    released_pressure
}

fn populate_room_graph(rooms: &mut Vec<Room>) {
    let room_clone = rooms.clone();
    for room in rooms {
        let paths = shortest_path(&room_clone, room.name.clone());
        room.shortest_paths = paths;
    }
}

fn shortest_path(rooms: &Vec<Room>, start: String) -> Vec<(String, u32)> {
    let mut visited: HashMap<String, u32> = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        cost: 0,
        position: start.clone(),
    });

    while let Some(current) = priority_queue.pop() {
        let mut skip = false;
        visited
            .entry(current.position.clone())
            .and_modify(|known_shortest| {
                if current.cost < *known_shortest {
                    *known_shortest = current.cost;
                } else {
                    skip = true;
                }
            })
            .or_insert(current.cost);
        if skip {
            continue;
        }
        let neighbors = rooms
            .iter()
            .find(|r| r.name == current.position)
            .unwrap()
            .tunnels
            .clone();
        for n in neighbors {
            priority_queue.push(State {
                cost: current.cost + 1,
                position: n,
            })
        }
    }
    let mut result: Vec<_> = visited.into_iter().filter(|v| v.0 != start).collect();
    result.sort();
    result
}

impl Room {
    fn dist_to(&self, other: &str) -> i32 {
        self.shortest_paths.iter().find(|p| p.0 == other).unwrap().1 as i32
    }
}

fn bfs(rooms: &Vec<Room>, current_path: Vec<String>, remaining_time: u32) -> Vec<Vec<String>> {
    let mut result = vec![];
    let current_room = rooms
        .iter()
        .find(|r| r.name == *current_path.last().unwrap())
        .unwrap();

    for (r, dist) in &current_room.shortest_paths {
        let next_room = rooms.iter().find(|r1| r1.name == *r).unwrap();
        if *dist >= remaining_time || current_path.contains(&r) || next_room.flow_rate == 0 {
            continue;
        }
        let mut next_path = current_path.clone();
        next_path.push(r.clone());
        let nested_paths = bfs(rooms, next_path.clone(), remaining_time - dist - 1);
        if nested_paths.is_empty() {
            result.push(next_path);
        } else {
            result.extend(nested_paths.into_iter());
        }
    }
    result
}

pub fn solve1() {
    let mut rooms = read_input();
    populate_room_graph(&mut rooms);
    let steps = bfs(&rooms, vec!["AA".to_string()], 30);
    let result = steps
        .into_iter()
        .map(|steps| eval(&rooms, steps))
        .max()
        .unwrap();
    println!("{result}")
    //eval(rooms)

    // let mut remaining_time = 30;
    // let mut current_room = "AA";
    // let mut solution = vec![];
    // let mut seen = vec![];
    //
    // while remaining_time > 0 {
    //     seen.push(current_room);
    //     let (room, pressure) = rooms
    //         .iter()
    //         .filter(|r| !seen.contains(&&*r.name))
    //         .map(|r| {
    //             (
    //                 r,
    //                 (remaining_time - r.dist_to(current_room) - 1) * r.flow_rate as i32,
    //             )
    //         })
    //         .max_by_key(|r| r.1)
    //         .unwrap();
    //     if pressure == 0 {
    //         break;
    //     }
    //     dbg!(room.dist_to(current_room));
    //     solution.push(room.name.clone());
    //     let dist = room.dist_to(current_room);
    //     remaining_time -= dist + 1;
    //     current_room = &room.name;
    // }
    // let result = eval(&rooms, dbg!(solution));
}

#[cfg(test)]
mod tests {
    use crate::day16::*;

    #[test]
    fn it_works() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB, CC
        Valve BB has flow rate=13; tunnels lead to valves AA, DD
        Valve CC has flow rate=2; tunnels lead to valves AA, DD, EE
        Valve DD has flow rate=2; tunnels lead to valves CC, BB, EE
        Valve EE has flow rate=2; tunnels lead to valves DD, CC";
        let mut rooms = parse_input(input.to_string());
        populate_room_graph(&mut rooms);
        let result = eval(
            &rooms,
            vec!["BB".to_string(), "EE".to_string(), "CC".to_string()],
        );
        assert_eq!(result, 28 * 13 + 25 * 2 + 23 * 2)
    }
}
