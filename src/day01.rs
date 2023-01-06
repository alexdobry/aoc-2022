use std::fs;

fn read_input() -> Vec<u32> {
  let file_path = "inputs/day01_1.txt";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  contents
    .split("\n\n")
    .map(|line| line.lines()
      .filter_map(|a| a.parse::<u32>().ok())
      .sum()
    )
    .collect()
}

fn read_input_imp() -> Vec<u32> {
  let file_path = "inputs/day01_1.txt";
  let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
  let mut result: Vec<u32> = vec![0];
  for content in contents.lines() {
    match content.parse::<u32>() {
      Ok(n) => {
        let len = result.len();
        result[len - 1] += n;
      }
      Err(_) => {
        result.push(0);
      }
    }
  }
  result
}

pub fn solve1() {
  let result = read_input()
    .into_iter()
    .max()
    .unwrap_or(0);
  println!("{result}");
}

pub fn solve1_imp() {
  let mut result = 0;
  let input = read_input_imp();
  for n in input {
    result = n.max(result)
  }
  println!("{result}")
}

pub fn solve2() {
  let mut input: Vec<u32> = read_input();
  input.sort();
  let result: u32 = input
    .into_iter()
    .rev()
    .take(3)
    .sum();
  println!("{result}");
}

pub fn solve2_imp() {
  let mut tmp = [0, 0, 0, 0];
  let input = read_input_imp();
  for n in input {
    tmp[0] = n;
    tmp.sort();
  }
  let result = tmp[1] + tmp[2] + tmp[3];
  println!("{result}")
}