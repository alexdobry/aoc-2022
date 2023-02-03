use std::fs;

fn read_input() -> String {
    let file_path = "inputs/day06.txt";
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

fn is_distinct(slice: &[char]) -> bool {
    let mut acc: u32 = 0;
    for char in slice {
        acc |= 1 << *char as u32 - 96;
    }
    return acc.count_ones() == slice.len() as u32;
}

fn solve(window_size: usize) {
    let input = read_input();
    let chars: Vec<char> = input.chars().collect();
    for (i, window) in chars.windows(window_size).enumerate() {
        if is_distinct(window) {
            let result = i + window_size;
            println!("{}", result);
            break;
        }
    }
}

pub fn solve1() {
    solve(4);
}

pub fn solve2() {
    solve(14);
}

#[cfg(test)]
mod tests {
    use crate::day06::is_distinct;

    #[test]
    fn it_works() {
        assert_eq!(is_distinct(&['a', 'b', 'c', 'd']), true);
        assert_eq!(is_distinct(&['a', 'b', 'a', 'd']), false);
    }
}
