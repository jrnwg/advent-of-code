use advent_of_code::{get_input, solve};

fn main() {
    let input = get_input(10);
    solve(&input, part_one, part_two);
}

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<bool>>) {
    let mut target: Vec<bool> = Vec::new();
    let mut buttons: Vec<Vec<bool>> = Vec::new();

    line.split_whitespace().for_each(|part| {
        if part.starts_with("[") {
            target = part[1..part.len() - 1].chars().map(|c| c == '#').collect();
        } else if part.starts_with("(") {
            let mut b = vec![false; target.len()];
            part[1..part.len() - 1]
                .split(',')
                .for_each(|i| b[i.parse::<usize>().unwrap()] = true);
            buttons.push(b);
        }
    });

    (target, buttons)
}

fn dfs(
    target: &Vec<bool>,
    state: &Vec<bool>,
    buttons: &[Vec<bool>],
    count: &usize,
) -> Option<usize> {
    if state == target {
        return Some(*count);
    }

    if buttons.is_empty() {
        return None;
    }

    let skip = dfs(target, state, &buttons[1..], count);

    let new_state: Vec<bool> = state
        .iter()
        .zip(buttons[0].iter())
        .map(|(s, b)| s ^ b)
        .collect();
    let press = dfs(target, &new_state, &buttons[1..], &(count + 1));

    match (skip, press) {
        (Some(s), Some(p)) => Some(s.min(p)),
        (Some(s), None) => Some(s),
        (None, Some(p)) => Some(p),
        (None, None) => None,
    }
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (target, buttons) = parse_line(line);
            dfs(&target, &vec![false; target.len()], &buttons, &0).unwrap_or(usize::MAX)
        })
        .sum()
}

fn part_two(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 7);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 0);
    }
}
