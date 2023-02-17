use std::fs;

fn read_input() -> Vec<Vec<i32>> {
    let file_path = "inputs/day08.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    parse_input(input)
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

struct Directions<'a> {
    left: &'a mut dyn Iterator<Item = i32>,
    right: &'a mut dyn Iterator<Item = i32>,
    top: &'a mut dyn Iterator<Item = i32>,
    bottom: &'a mut dyn Iterator<Item = i32>,
}

fn iterate_grid<F>(input: Vec<Vec<i32>>, mut k: F)
where
    F: FnMut(i32, &mut Directions),
{
    for (row_index, row) in input.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            let mut left = row[..col_index].iter().copied().rev();
            let mut right = row[col_index + 1..].iter().copied();
            let mut top = (0..row_index).map(|ri| input[ri][col_index]).rev();
            let mut bot = (row_index + 1..input.len()).map(|ri| input[ri][col_index]);
            k(
                *value,
                &mut Directions {
                    left: &mut left,
                    right: &mut right,
                    top: &mut top,
                    bottom: &mut bot,
                },
            )
        }
    }
}

fn count_visible_trees(input: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    iterate_grid(
        input,
        |value,
         Directions {
             left,
             right,
             top,
             bottom,
         }| {
            let left_max = left.max().unwrap_or(-1);
            let right_max = right.max().unwrap_or(-1);
            let top_max = top.max().unwrap_or(-1);
            let bot_max = bottom.max().unwrap_or(-1);
            let min_height = left_max.min(right_max).min(top_max).min(bot_max);
            if value > min_height {
                result += 1;
            }
        },
    );
    result
}

fn find_best_scenic_score(input: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    iterate_grid(
        input,
        |value,
         Directions {
             left,
             right,
             top,
             bottom,
         }| {
            let score = find_scenic_score(value, left)
                * find_scenic_score(value, right)
                * find_scenic_score(value, top)
                * find_scenic_score(value, bottom);
            result = result.max(score);
        },
    );
    result
}

fn find_scenic_score(value: i32, iter: &mut &mut dyn Iterator<Item = i32>) -> i32 {
    let mut res = 0;
    for x in iter {
        res += 1;
        if x >= value {
            break;
        }
    }
    res
}

pub fn solve1() {
    let input = read_input();
    let result = count_visible_trees(input);
    println!("{result}")
}

pub fn solve2() {
    let input = read_input();
    let result = find_best_scenic_score(input);
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use crate::day08::count_visible_trees;
    use crate::day08::parse_input;

    #[test]
    fn it_works() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let grid = parse_input(input.to_string());
        /*println!("{grid:#?}");*/
        let result = count_visible_trees(grid);
        println!("{result}");
    }
}
