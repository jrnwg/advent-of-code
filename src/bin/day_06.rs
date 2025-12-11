use std::collections::HashMap;

use advent_of_code::{get_input, solve};

fn main() {
    let input = get_input(6);
    solve(&input, part_one, part_two);
}
fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let rows = lines.len() - 1;
    let flattened = lines
        .iter()
        .take(rows)
        .flat_map(|line| {
            line.split_whitespace()
                .filter_map(|p| p.parse::<usize>().ok())
        })
        .collect::<Vec<usize>>();

    let operators = lines
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let cols = flattened.len() / rows;
    (0..cols)
        .map(|i| match operators[i] {
            "*" => flattened[i..].iter().step_by(cols).product::<usize>(),
            "+" => flattened[i..].iter().step_by(cols).sum::<usize>(),
            _ => unreachable!(),
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();

    let rows = lines.len() - 1;
    let chars: HashMap<(usize, usize), char> = lines
        .iter()
        .take(rows)
        .enumerate()
        .flat_map(|(idx, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_whitespace())
                .map(move |(jdx, c)| ((idx, jdx), c))
        })
        .collect();

    lines
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(idx, c)| {
            let mut numbers: Vec<usize> = Vec::new();
            let mut dx = 0;
            loop {
                let mut n_chars: Vec<char> = Vec::new();
                for y in 0..=rows {
                    if chars.contains_key(&(y, idx + dx)) {
                        n_chars.push(*chars.get(&(y, idx + dx)).unwrap());
                    }
                }
                if n_chars.is_empty() {
                    break;
                }
                numbers.push(n_chars.iter().collect::<String>().parse::<usize>().unwrap());
                dx += 1;
            }

            match c {
                '*' => numbers.iter().product::<usize>(),
                '+' => numbers.iter().sum::<usize>(),
                _ => unreachable!(),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 3263827);
    }
}
