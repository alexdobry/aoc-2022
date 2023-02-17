use std::fs;

enum Shape {
  Rock,
  Paper,
  Scissor,
}

#[derive(PartialEq)]
enum Outcome {
  Win,
  Loss,
  Draw,
}

impl Shape {
  fn fight(&self, other: &Shape) -> Outcome {
    match (self, other) {
      (Shape::Rock, Shape::Paper) => Outcome::Loss,
      (Shape::Rock, Shape::Scissor) => Outcome::Win,
      (Shape::Rock, Shape::Rock) => Outcome::Draw,
      (Shape::Paper, Shape::Scissor) => Outcome::Loss,
      (Shape::Paper, Shape::Rock) => Outcome::Win,
      (Shape::Paper, Shape::Paper) => Outcome::Draw,
      (Shape::Scissor, Shape::Rock) => Outcome::Loss,
      (Shape::Scissor, Shape::Paper) => Outcome::Win,
      (Shape::Scissor, Shape::Scissor) => Outcome::Draw
    }
  }

  fn yields(&self, outcome: &Outcome) -> Shape {
    for shape in [Shape::Rock, Shape::Paper, Shape::Scissor] {
      if &shape.fight(self) == outcome { return shape; }
    }
    unreachable!()
  }

  fn score(&self) -> u32 {
    match self {
      Shape::Rock => 1,
      Shape::Paper => 2,
      Shape::Scissor => 3
    }
  }

  fn parse(str: &str) -> Shape {
    match str {
      "A" | "X" => Shape::Rock,
      "B" | "Y" => Shape::Paper,
      "C" | "Z" => Shape::Scissor,
      _ => panic!()
    }
  }
}

impl Outcome {
  fn score(&self) -> u32 {
    match self {
      Outcome::Win => 6,
      Outcome::Draw => 3,
      Outcome::Loss => 0,
    }
  }

  fn parse(str: &str) -> Outcome {
    match str {
      "X" => Outcome::Loss,
      "Y" => Outcome::Draw,
      "Z" => Outcome::Win,
      _ => panic!()
    }
  }
}

fn read_input() -> Vec<(Shape, String)> {
  let file_path = "inputs/day02_1.txt";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let mut result = Vec::new();
  for line in contents.lines() {
    let (left, right) = line.split_once(' ').unwrap();
    result.push((Shape::parse(left), right.into()))
  }
  result
}

pub fn solve1() {
  let rounds = read_input();
  let mut result = 0;
  for (them, right_side) in rounds {
    let us = Shape::parse(&right_side);
    let outcome = us.fight(&them);
    result += outcome.score() + us.score()
  }
  println!("{result}");
}

pub fn solve2() {
  let rounds = read_input();
  let mut result = 0;
  for (them, right_side) in rounds {
    let outcome = Outcome::parse(&right_side);
    let us = them.yields(&outcome);
    result += outcome.score() + us.score()
  }
  println!("{result}");
}
