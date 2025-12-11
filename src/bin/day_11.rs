use std::collections::HashMap;

use advent_of_code::{get_input, solve};
use cached::proc_macro::cached;

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

#[cached(
    size = 1000,
    key = "(String, Vec<(String, bool)>)",
    convert = r#"{
        let mut seen_vec: Vec<(String, bool)> = seen.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        seen_vec.sort();
        (device.to_string(), seen_vec)
    }"#
)]
fn dfs(factory: &HashMap<&str, Vec<&str>>, device: &str, seen: &HashMap<String, bool>) -> usize {
    if device == "out" {
        if seen.values().all(|&v| v) {
            return 1;
        } else {
            return 0;
        }
    }

    if seen.contains_key(device) {
        let mut updated = seen.clone();
        updated.insert(device.to_string(), true);

        return factory
            .get(device)
            .unwrap()
            .iter()
            .map(|d| dfs(factory, d, &updated))
            .sum();
    };

    factory
        .get(device)
        .unwrap()
        .iter()
        .map(|d| dfs(factory, d, seen))
        .sum()
}

fn part_one(input: &str) -> usize {
    let factory = init_factory(input);
    dfs(&factory, "you", &HashMap::new())
}

fn part_two(input: &str) -> usize {
    let factory = init_factory(input);
    dfs(
        &factory,
        "svr",
        &HashMap::from([("dac".to_string(), false), ("fft".to_string(), false)]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT_1), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT_2), 2);
    }
}
