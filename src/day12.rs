use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vertex>,
}

#[derive(Debug)]
struct Vertex {
    edges: Vec<usize>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
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

impl Graph {
    fn neighbors(&self, ix: usize) -> &Vec<usize> {
        &self.vertices[ix].edges
    }

    fn transpose(self) -> Self {
        let mut new_vertices = vec![];
        for _ in 0..self.vertices.len() {
            new_vertices.push(Vertex { edges: vec![] })
        }
        for (i, v) in self.vertices.into_iter().enumerate() {
            for n in v.edges {
                new_vertices[n].edges.push(i);
            }
        }
        Graph {
            vertices: new_vertices,
        }
    }
}

fn read_input() -> (Graph, Vec<Vec<i32>>, usize, usize) {
    let file_path = "inputs/day12.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn char_to_height(c: char) -> i32 {
    if c == 'S' {
        return 0;
    }
    if c == 'E' {
        return 25;
    }
    c as i32 - 97
}

fn parse_input(input: String) -> (Graph, Vec<Vec<i32>>, usize, usize) {
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut start = None;
    let mut end = None;

    for row in input.lines() {
        let row_len = row.len();
        let mut new_row = vec![];
        for (column, height) in row.chars().enumerate() {
            if height == 'S' {
                start = Some(row_len * grid.len() + column);
            }
            if height == 'E' {
                end = Some(row_len * grid.len() + column);
            }
            new_row.push(char_to_height(height));
        }
        grid.push(new_row);
    }

    let start = start.unwrap();
    let end = end.unwrap();
    let row_len = grid[0].len();

    let mut vertices = vec![];
    for i in 0..grid.len() {
        for j in 0..row_len {
            let mut edges = vec![];
            let current_height = grid[i][j];
            if j > 0 {
                let left_height = grid[i][j - 1];
                if current_height + 1 >= left_height {
                    edges.push(i * row_len + j - 1);
                }
            }
            if let Some(right_height) = grid[i].get(j + 1) {
                if current_height + 1 >= *right_height {
                    edges.push(i * row_len + j + 1);
                }
            }
            if i > 0 {
                let top_height = grid[i - 1][j];
                if current_height + 1 >= top_height {
                    edges.push((i - 1) * row_len + j);
                }
            }
            if let Some(bot_height) = grid.get(i + 1).map(|g| g[j]) {
                if current_height + 1 >= bot_height {
                    edges.push((i + 1) * row_len + j);
                }
            }
            vertices.push(Vertex { edges });
        }
    }

    (Graph { vertices }, grid, start, end)
}

fn shortest_path(graph: Graph, start: usize, stop: impl Fn(usize) -> bool) -> usize {
    let mut visited: HashMap<usize, usize> = HashMap::new();
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        cost: 0,
        position: start,
    });

    loop {
        let current = priority_queue.pop().unwrap();
        let mut skip = false;
        if stop(current.position) {
            return current.cost;
        }
        visited
            .entry(current.position)
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
        let neighbors = graph.neighbors(current.position);
        for n in neighbors {
            priority_queue.push(State {
                cost: current.cost + 1,
                position: *n,
            })
        }
    }
}

pub fn solve1() {
    let (graph, _, start, end) = read_input();
    let result = shortest_path(graph, start, |pos| pos == end);
    println!("{result}");
}

pub fn solve2() {
    let (graph, grid, _, end) = read_input();
    let result = shortest_path(graph.transpose(), end, |pos| {
        grid[pos / grid[0].len()][pos % grid[0].len()] == 0
    });
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::day12::parse_input;
    use crate::day12::shortest_path;

    #[test]
    fn it_works() {
        let input = "abcSE";
        let (g, _, s, e) = parse_input(input.to_string());
        dbg!(&g);
        dbg!(g.transpose());
    }
}
