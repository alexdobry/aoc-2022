use std::fs;

#[derive(Debug, Clone, Copy)]
struct Snafu(pub i64);

impl Snafu {
    fn parse(line: &str) -> Snafu {
        let mut current_place = 1;
        let mut result = 0i64;
        for char in line.chars().rev() {
            let modify = match char {
                '0' => 0,
                '1' => current_place,
                '2' => current_place * 2,
                '-' => -current_place,
                '=' => -(current_place * 2),
                _ => unreachable!(),
            };
            result += modify;
            current_place *= 5;
        }
        Snafu(result)
    }

    fn print(&self) -> String {
        let mut res = String::new();
        let mut current = self.0;
        let mut carry = 0;
        while current != 0 {
            let remainder = current % 5;
            carry = match carry + remainder {
                0 => {
                    res.push('0');
                    0
                }
                1 => {
                    res.push('1');
                    0
                }
                2 => {
                    res.push('2');
                    0
                }
                3 => {
                    res.push('=');
                    1
                }
                4 => {
                    res.push('-');
                    1
                }
                5 => {
                    res.push('0');
                    1
                }
                _ => unreachable!(),
            };
            current /= 5;
        }
        if carry == 1 {
            res.push('1')
        }
        res.chars().rev().collect()
    }
}

fn read_input() -> Vec<Snafu> {
    let file_path = "inputs/day25.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Snafu> {
    let mut snafus = vec![];
    for line in input.lines() {
        snafus.push(Snafu::parse(line));
    }
    snafus
}

pub fn solve1() {
    let result: String = Snafu(read_input().iter().map(|s| s.0).sum()).print();
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::day25::*;

    #[test]
    fn it_works() {
        // 0 => 0
        // 1 => 1
        // 2 => 2
        // 3 => -2 + uebertrag 1
        // 4 => -1 + uebertrag 1
        let test_cases = vec![
            (1, "1"),
            (2, "2"),
            (3, "1="),
            (4, "1-"),
            (5, "10"),
            (6, "11"),
            (7, "12"),
            (8, "2="),
            (9, "2-"),
            (10, "20"),
            (15, "1=0"),
            (20, "1-0"),
            (24, "10-"),
            (124, "100-"),
            (2022, "1=11-2"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];

        for (expected, input) in test_cases {
            let snafu = Snafu::parse(input);
            assert_eq!(expected, snafu.0);
            assert_eq!(input, &snafu.print());
        }
    }
}
