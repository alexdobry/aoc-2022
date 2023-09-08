use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    pub fn new(x: i32, y: i32, z: i32) -> Cube {
        Cube { x, y, z }
    }

    pub fn neighbors(&self) -> [Cube; 6] {
        [
            Cube::new(self.x + 1, self.y, self.z),
            Cube::new(self.x, self.y + 1, self.z),
            Cube::new(self.x, self.y, self.z + 1),
            Cube::new(self.x - 1, self.y, self.z),
            Cube::new(self.x, self.y - 1, self.z),
            Cube::new(self.x, self.y, self.z - 1),
        ]
    }
}

fn read_input() -> Vec<Cube> {
    let file_path = "inputs/day18.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Cube> {
    let mut cubes = vec![];
    for line in input.lines() {
        let (x, rest) = line.split_once(',').unwrap();
        let (y, z) = rest.split_once(',').unwrap();
        cubes.push(Cube {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        })
    }
    cubes
}

pub fn solve1() {
    let cubes = read_input();
    let mut surface_area = 0;

    for cube in cubes.iter() {
        let neighboring_cubes = cube.neighbors();
        for nbc in neighboring_cubes.iter() {
            if !cubes.contains(nbc) {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");
}

#[derive(Default)]
struct Bounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub min_z: i32,
    pub max_z: i32,
}

struct Solve2 {
    world: HashSet<Cube>,
    bounds: Bounds,
    is_trapped: HashSet<Cube>,
    is_free: HashSet<Cube>,
}

impl Solve2 {
    pub fn new() -> Self {
        let mut world = HashSet::new();
        let mut bounds: Bounds = Default::default();
        for cube in read_input() {
            bounds.min_x = cube.x.min(bounds.min_x);
            bounds.min_y = cube.y.min(bounds.min_y);
            bounds.min_z = cube.z.min(bounds.min_z);
            bounds.max_x = cube.x.max(bounds.max_x);
            bounds.max_y = cube.y.max(bounds.max_y);
            bounds.max_z = cube.z.max(bounds.max_z);
            world.insert(cube);
        }
        Solve2 {
            world,
            bounds,
            is_trapped: HashSet::new(),
            is_free: HashSet::new(),
        }
    }

    fn is_free(&self, cube: &Cube) -> bool {
        if self.is_free.contains(cube) {
            true
        } else if cube.x < self.bounds.min_x
            || cube.x > self.bounds.max_x
            || cube.y < self.bounds.min_y
            || cube.y > self.bounds.max_y
            || cube.z < self.bounds.min_z
            || cube.z > self.bounds.max_z
        {
            true
        } else {
            false
        }
    }

    fn is_trapped(&self, cube: &Cube) -> bool {
        self.is_trapped.contains(cube)
    }

    // https://en.wikipedia.org/wiki/Component_(graph_theory)#Algorithms
    fn escape(&mut self, cube: &Cube) -> bool {
        let mut visited: HashSet<Cube> = HashSet::new();
        let mut to_explore: Vec<Cube> = vec![cube.clone()];

        while let Some(next) = to_explore.pop() {
            if self.world.contains(&next) {
                continue;
            }
            if self.is_free(&next) {
                self.is_free.extend(visited);
                return true;
            }
            if self.is_trapped(&next) {
                self.is_trapped.extend(visited);
                return false;
            }
            to_explore.extend(
                next.neighbors()
                    .into_iter()
                    .filter(|n| !visited.contains(n)),
            );
            visited.insert(next);
        }

        self.is_trapped.extend(visited);
        return false;
    }

    pub fn solve2(&mut self) {
        let mut surface_area = 0;

        for cube in self.world.clone().iter() {
            let neighboring_cubes = cube.neighbors();
            for nbc in neighboring_cubes.iter() {
                if !self.world.contains(nbc) && self.escape(nbc) {
                    surface_area += 1;
                }
            }
        }

        println!("{surface_area}");
    }
}

pub fn solve2() {
    Solve2::new().solve2()
}

fn test_cubes() -> Vec<Cube> {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    parse_input(input.to_string())
}
