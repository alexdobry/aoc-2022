use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

use rayon::prelude::*;

#[derive(Clone, Debug)]
struct Room {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
    shortest_paths: Vec<(String, i32)>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i32,
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

fn eval(rooms: &[Room], mut steps: Vec<String>, max_time: i32) -> i32 {
    steps.reverse();
    assert_eq!(steps.pop().unwrap(), "AA");
    let mut released_pressure: i32 = 0;
    let mut open_valves = HashSet::new();
    let mut current_room = "AA".to_string();
    let mut minute_counter = 0;

    while minute_counter < max_time {
        let open_pressure = rooms
            .iter()
            .map(|r| {
                if open_valves.contains(r.name.as_str()) {
                    r.flow_rate
                } else {
                    0
                }
            })
            .sum::<i32>();

        match steps.pop() {
            None => {
                released_pressure += (max_time - minute_counter) * open_pressure;
                break;
            }
            Some(step) => {
                let room = rooms.iter().find(|r| r.name == current_room).unwrap();
                let distance = match room.shortest_paths.iter().find(|ps| ps.0 == step) {
                    None => {
                        // room == AA == step
                        dbg!(&room);
                        dbg!(&step);
                        dbg!(&steps);
                        0
                    }
                    Some(res) => res.1,
                };
                minute_counter += distance + 1;
                assert!(minute_counter <= max_time);
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

fn shortest_path(rooms: &[Room], start: String) -> Vec<(String, i32)> {
    let mut visited: HashMap<String, i32> = HashMap::new();
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
        self.shortest_paths.iter().find(|p| p.0 == other).unwrap().1
    }
}

fn path_length(rooms: &[Room], path: &[String]) -> i32 {
    let mut length = 0;
    let mut current: &Room = path
        .first()
        .and_then(|p| rooms.iter().find(|r| r.name == *p))
        .unwrap();
    for path in path.iter().skip(1) {
        length += current.dist_to(path) + 1;
        current = rooms.iter().find(|r| r.name == *path).unwrap();
    }
    length
}

fn bfs(rooms: &Vec<Room>, current_path: Vec<String>, max_time: i32) -> Vec<Vec<String>> {
    let eligible_rooms = rooms
        .iter()
        .filter(|r| !current_path.contains(&r.name) && r.flow_rate > 0);
    let new_paths: Vec<Vec<String>> = eligible_rooms
        .map(|r| {
            let mut current_path = current_path.clone();
            current_path.push(r.name.clone());
            current_path
        })
        .filter(|p| path_length(rooms, p) <= max_time)
        .collect();
    if new_paths.is_empty() {
        vec![current_path]
    } else {
        new_paths
            .into_par_iter()
            .flat_map(|p| bfs(rooms, p, max_time))
            .collect()
    }
}

pub fn solve1() {
    let max_time = 30;
    let mut rooms = read_input();
    populate_room_graph(&mut rooms);
    let paths = bfs(&rooms, vec!["AA".to_string()], max_time);
    let result = paths
        .into_par_iter()
        .map(|steps| eval(&rooms, steps, max_time))
        .max()
        .unwrap();
    println!("{result}");
}

pub fn solve2() {
    let max_time = 26;
    let mut rooms = read_input();
    populate_room_graph(&mut rooms);
    let paths = bfs(&rooms, vec!["AA".to_string()], max_time);
    let path_pairs: Vec<(Vec<String>, Vec<String>)> = paths
        .par_iter()
        .flat_map(|path1| {
            paths
                .iter()
                .filter(|path2| !path1.iter().any(|r| r != "AA" && path2.contains(r)))
                .map(|p2| (path1.clone(), p2.clone()))
                .collect::<Vec<_>>()
        })
        .collect();
    let result = path_pairs
        .into_par_iter()
        .map(|(steps1, steps2)| eval(&rooms, steps1, max_time) + eval(&rooms, steps2, max_time))
        .max()
        .unwrap();
    println!("{result}");
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
            30,
        );
        assert_eq!(result, 28 * 13 + 25 * 2 + 23 * 2)
    }
}
