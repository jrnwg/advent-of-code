use advent_of_code::get_input;
use std::collections::BTreeMap;

fn main() {
    let input = get_input(5);
    println!("Part 1: {}", part_one(&input));
}

struct RangeTree {
    ranges: BTreeMap<u64, u64>,
}

impl RangeTree {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn insert(&mut self, start: u64, end: u64) {
        if self.ranges.contains_key(&start) && self.ranges[&start] >= end {
            return;
        }
        self.ranges.insert(start, end);
    }

    fn contains(&self, value: u64) -> bool {
        self.ranges
            .iter()
            .any(|(&start, &end)| start <= value && value <= end)
    }
}

fn part_one(input: &str) -> u64 {
    let mut tree = RangeTree::new();
    let mut init = true;
    let mut sum = 0;

    input.lines().for_each(|line| {
        if line.is_empty() {
            init = false;
            return;
        }
        if init {
            let start_end: Vec<&str> = line.split('-').collect();
            let start: u64 = start_end[0].parse().unwrap();
            let end: u64 = start_end[1].parse().unwrap();

            tree.insert(start, end);
        } else {
            let value: u64 = line.parse().unwrap();
            if tree.contains(value) {
                sum += 1;
            }
        }
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 3);
    }
}
