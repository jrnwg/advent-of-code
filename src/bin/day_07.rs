use advent_of_code::{get_input, solve};

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = get_input(7);
    solve(&input, part_one, part_two);
}

fn part_one(input: &str) -> usize {
    let mut split = 0;
    let mut beams: HashSet<usize> =
        HashSet::from([input.lines().next().unwrap().find('S').unwrap()]);
    input.lines().skip(1).for_each(|line| {
        for (idx, c) in line.chars().enumerate() {
            if c == '^' && beams.contains(&idx) {
                split += 1;
                beams.insert(idx - 1);
                beams.insert(idx + 1);
                beams.remove(&idx);
            }
        }
    });

    split
}

fn part_two(input: &str) -> usize {
    let mut beams: HashMap<usize, usize> = HashMap::new();
    let idx = input.lines().next().unwrap().find('S').unwrap();
    beams.insert(idx, 1);

    for line in input.lines().skip(1) {
        for (idx, count) in beams.clone() {
            if line.chars().nth(idx).unwrap() == '^' {
                *beams.entry(idx - 1).or_insert(0) += count;
                *beams.entry(idx + 1).or_insert(0) += count;
                beams.remove(&idx);
            }
        }
    }

    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 40);
    }
}
