use std::cmp::Ordering;
use std::fs;

fn read_input() -> Vec<i64> {
    let file_path = "inputs/day20.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn materialize_numbers(numbers: &[i64], positions: &[usize]) -> Vec<i64> {
    let mut result: Vec<i64> = (0..numbers.len() as i64).collect();
    for (idx, pos) in positions.iter().enumerate() {
        result[*pos] = numbers[idx];
    }
    result
}

fn shift_pos(start_pos: i64, delta: i64, n: i64) -> i64 {
    let mut new_pos = start_pos;
    let mut delta = delta % (n - 1);
    while delta != 0 {
        if delta < 0 {
            let next_pos = new_pos - 1;
            new_pos = if next_pos == -1 {
                n - 2
            } else if next_pos == 0 {
                n - 1
            } else {
                next_pos
            };
            delta += 1;
        } else {
            let next_pos = new_pos + 1;
            new_pos = if next_pos == n { 1 } else { next_pos };
            delta -= 1;
        }
    }
    new_pos
}

pub fn solve1() {
    let numbers = read_input();
    let n = numbers.len();
    let mut positions: Vec<usize> = (0..n).collect();

    for (idx, number) in numbers.iter().enumerate() {
        let start_pos = positions[idx];
        let end_pos = shift_pos(start_pos as i64, *number, n as i64) as usize;
        match start_pos.cmp(&end_pos) {
            Ordering::Equal => {}
            Ordering::Less => positions.iter_mut().for_each(|p| {
                if (start_pos..=end_pos).contains(p) {
                    *p = p.checked_sub(1).unwrap_or(0);
                }
            }),
            Ordering::Greater => positions.iter_mut().for_each(|p| {
                if (end_pos..=start_pos).contains(p) {
                    *p += 1
                }
            }),
        }
        positions[idx] = end_pos;
    }
    let result_numbers = materialize_numbers(&numbers, &positions);
    let zero_idx = result_numbers.iter().position(|n| *n == 0).unwrap();
    let result = dbg!(result_numbers[(zero_idx + 1000) % n])
        + dbg!(result_numbers[(zero_idx + 2000) % n])
        + dbg!(result_numbers[(zero_idx + 3000) % n]);
    println!("{result}");
}

pub fn solve2() {
    let decryption_key = 811589153;
    let numbers: Vec<i64> = read_input().iter().map(|n| n * decryption_key).collect();
    let n = numbers.len();
    let mut positions: Vec<usize> = (0..n).collect();

    for _ in 0..10 {
        for (idx, number) in numbers.iter().enumerate() {
            let start_pos = positions[idx];
            let end_pos = shift_pos(start_pos as i64, *number, n as i64) as usize;
            match start_pos.cmp(&end_pos) {
                Ordering::Equal => {}
                Ordering::Less => positions.iter_mut().for_each(|p| {
                    if (start_pos..=end_pos).contains(p) {
                        *p = p.checked_sub(1).unwrap_or(0);
                    }
                }),
                Ordering::Greater => positions.iter_mut().for_each(|p| {
                    if (end_pos..=start_pos).contains(p) {
                        *p += 1
                    }
                }),
            }
            positions[idx] = end_pos;
        }
    }
    let result_numbers = materialize_numbers(&numbers, &positions);
    let zero_idx = result_numbers.iter().position(|n| *n == 0).unwrap();
    let result = dbg!(result_numbers[(zero_idx + 1000) % n])
        + dbg!(result_numbers[(zero_idx + 2000) % n])
        + dbg!(result_numbers[(zero_idx + 3000) % n]);
    println!("{result}");
}
