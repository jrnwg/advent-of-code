use advent_of_code::{get_input, solve};
use std::collections::BTreeMap;

fn main() {
    let input = get_input(5);
    solve(&input, part_one, part_two);
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

    fn merge(&mut self) {
        let mut merged: BTreeMap<u64, u64> = BTreeMap::new();

        for (&start, &end) in &self.ranges {
            if let Some((&last_start, &last_end)) = merged.iter().next_back() {
                if start <= last_end + 1 {
                    merged.insert(last_start, last_end.max(end));
                } else {
                    merged.insert(start, end);
                }
            } else {
                merged.insert(start, end);
            }
        }

        self.ranges = merged;
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

fn part_two(input: &str) -> u64 {
    let mut tree = RangeTree::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let start_end: Vec<&str> = line.split('-').collect();
        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();

        tree.insert(start, end);
    }

    tree.merge();

    let mut sum = 0;
    for (start, end) in &tree.ranges {
        sum += end - start + 1;
    }
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

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 14);
    }
}
