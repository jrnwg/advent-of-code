use regex::Regex;
use std::collections::HashSet;

use advent_of_code::{get_input, solve_one};

fn main() {
    let input = get_input(12);
    solve_one(&input, part_one);
}

type Shape = HashSet<(usize, usize)>;
type Region = (usize, usize, Vec<usize>);

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    let re_shape = Regex::new(r"^\d:").unwrap();
    let re_region = Regex::new(r"^(\d+)x(\d+):\s(.*)").unwrap();

    let mut lines_iter = input.lines().peekable();
    while let Some(line) = lines_iter.next() {
        if re_shape.is_match(line) {
            let mut shape: HashSet<(usize, usize)> = HashSet::new();
            for (dy, line) in lines_iter.by_ref().enumerate() {
                if line.is_empty() {
                    break;
                }
                for (dx, c) in line.chars().enumerate() {
                    if c == '#' {
                        shape.insert((dx, dy));
                    }
                }
            }
            shapes.push(shape);
        } else if let Some(captures) = re_region.captures(line) {
            regions.push((
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap(),
                captures[3]
                    .split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            ));
        }
    }

    (shapes, regions)
}

fn part_one(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);

    regions
        .iter()
        .map(|(width, height, counts)| {
            let area = width * height;
            if area >= counts.iter().sum::<usize>() * 9 {
                return 1;
            }

            let needed = counts
                .iter()
                .enumerate()
                .map(|(idx, count)| shapes[idx].len() * count)
                .sum::<usize>();
            if area < needed {
                return 0;
            }

            panic!("Too complicated");
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    #[should_panic(expected = "Too complicated")]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }
}
