use std::collections::HashMap;

use advent_of_code::{get_input, solve};

fn main() {
    let input = get_input(11);
    solve(&input, part_one, part_two);
}

fn init_factory(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut factory: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let device = parts[0];
        let connections: Vec<&str> = parts[1].split_whitespace().collect();
        factory.insert(device, connections);
    }

    factory
}

fn dfs(factory: &HashMap<&str, Vec<&str>>, device: &str) -> usize {
    if device == "out" {
        return 1;
    }

    factory
        .get(device)
        .unwrap()
        .iter()
        .map(|d| dfs(factory, d))
        .sum()
}

fn part_one(input: &str) -> usize {
    let factory = init_factory(input);
    dfs(&factory, "you")
}

fn part_two(_input: &str) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 0);
    }
}
