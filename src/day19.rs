use std::fs;

use crate::day19::Resource::{Clay, Geode, Obsidian, Ore};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn all() -> Vec<Resource> {
        vec![Ore, Clay, Obsidian, Geode]
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i64,
    ore: i64,
    clay: i64,
    obsidian: (i64, i64),
    geode: (i64, i64),
}

impl Blueprint {
    fn max_ore_consumption(&self) -> i64 {
        self.ore
            .max(self.clay)
            .max(self.obsidian.0)
            .max(self.geode.0)
    }

    fn max_clay_consumption(&self) -> i64 {
        self.obsidian.1
    }

    fn max_obsidian_consumption(&self) -> i64 {
        self.geode.1
    }
}

fn read_input() -> Vec<Blueprint> {
    let file_path = "inputs/day19.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    for line in input.lines() {
        let (_, line) = line.split_once("Blueprint ").unwrap();
        let (id, line) = line.split_once(": Each ore robot costs ").unwrap();
        let (ore, line) = line.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay, line) = line.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_ore, line) = line.split_once(" ore and ").unwrap();
        let (obsidian_clay, line) = line.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_ore, line) = line.split_once(" ore and ").unwrap();
        let (geode_obsidian, _) = line.split_once(" obsidian.").unwrap();
        blueprints.push(Blueprint {
            id: id.parse().unwrap(),
            ore: ore.parse().unwrap(),
            clay: clay.parse().unwrap(),
            obsidian: (
                obsidian_ore.parse().unwrap(),
                obsidian_clay.parse().unwrap(),
            ),
            geode: (geode_ore.parse().unwrap(), geode_obsidian.parse().unwrap()),
        })
    }
    blueprints
}

struct Evaluator {
    blueprint: Blueprint,
    plan: Vec<Resource>,
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
    ore_robot: i64,
    clay_robot: i64,
    obsidian_robot: i64,
    geode_robot: i64,
}

struct EvalResult {
    produced_geodes: i64,
    finished: bool,
    ore_saturated: bool,
    clay_saturated: bool,
    obsidian_saturated: bool,
}

impl EvalResult {
    fn is_saturated(&self, resource: &Resource) -> bool {
        match resource {
            Ore => self.ore_saturated,
            Clay => self.clay_saturated,
            Obsidian => self.obsidian_saturated,
            Geode => false,
        }
    }
}

impl Evaluator {
    fn new(blueprint: Blueprint, mut plan: Vec<Resource>) -> Evaluator {
        plan.reverse();
        Evaluator {
            blueprint,
            plan,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }

    fn eval(&mut self, minutes: i64) -> EvalResult {
        let mut ore_saturated = false;
        let mut clay_saturated = false;
        let mut obsidian_saturated = false;
        let mut captured = false;
        for i in 0..minutes {
            self.tick();
            if !captured && self.plan.is_empty() {
                captured = true;
                let remaining_time = minutes - i - 1;
                ore_saturated = self.ore + self.ore_robot * remaining_time
                    > remaining_time * self.blueprint.max_ore_consumption();
                clay_saturated = self.clay + self.clay_robot * remaining_time
                    > remaining_time * self.blueprint.max_clay_consumption();
                obsidian_saturated = self.obsidian + self.obsidian_robot * remaining_time
                    > remaining_time * self.blueprint.max_obsidian_consumption();
            }
        }
        EvalResult {
            produced_geodes: self.geode,
            finished: self.plan.is_empty(),
            ore_saturated,
            clay_saturated,
            obsidian_saturated,
        }
    }

    fn tick(&mut self) {
        let new_ore = self.ore_robot;
        let new_clay = self.clay_robot;
        let new_obsidian = self.obsidian_robot;
        let new_geode = self.geode_robot;

        match self.plan.last() {
            None => {}
            Some(Ore) => {
                if self.ore >= self.blueprint.ore {
                    self.ore_robot += 1;
                    self.ore -= self.blueprint.ore;
                    self.plan.pop();
                }
            }
            Some(Clay) => {
                if self.ore >= self.blueprint.clay {
                    self.clay_robot += 1;
                    self.ore -= self.blueprint.clay;
                    self.plan.pop();
                }
            }
            Some(Obsidian) => {
                if self.ore >= self.blueprint.obsidian.0 && self.clay >= self.blueprint.obsidian.1 {
                    self.obsidian_robot += 1;
                    self.ore -= self.blueprint.obsidian.0;
                    self.clay -= self.blueprint.obsidian.1;
                    self.plan.pop();
                }
            }
            Some(Geode) => {
                if self.ore >= self.blueprint.geode.0 && self.obsidian >= self.blueprint.geode.1 {
                    self.geode_robot += 1;
                    self.ore -= self.blueprint.geode.0;
                    self.obsidian -= self.blueprint.geode.1;
                    self.plan.pop();
                }
            }
        }

        self.ore += new_ore;
        self.clay += new_clay;
        self.obsidian += new_obsidian;
        self.geode += new_geode;
    }
}

pub fn solve1() {
    let blueprints = read_input();
    let mut res = 0;
    for blueprint in blueprints {
        let (geodes, _) = find_best_plan(&blueprint, 24);
        res += blueprint.id * geodes;
        println!("finished {}", blueprint.id)
    }
    println!("result: {res}")
}

pub fn solve2() {
    let blueprints = read_input();
    let mut res = 1;
    for blueprint in blueprints.iter().take(3) {
        let (geodes, _) = find_best_plan(blueprint, 32);
        res *= geodes;
        println!("finished {}", blueprint.id)
    }
    println!("result: {res}")
}

fn find_best_plan(blueprint: &Blueprint, minutes: i64) -> (i64, Vec<Resource>) {
    let mut worklist: Vec<Vec<Resource>> = vec![vec![]];
    let mut max_geodes = 0;
    let mut best_plan = vec![];
    let mut count = 0;

    while let Some(current) = worklist.pop() {
        let res = Evaluator::new(blueprint.clone(), current.clone()).eval(minutes);
        if !res.finished {
            continue;
        }
        count += 1;
        for next_resource in Resource::all().into_iter().filter(|r| !res.is_saturated(r)) {
            let mut next_plan = current.clone();
            next_plan.push(next_resource);
            worklist.push(next_plan);
        }
        if res.produced_geodes > max_geodes {
            max_geodes = res.produced_geodes;
            best_plan = current;
        }
    }
    dbg!(count);
    (max_geodes, best_plan)
}
