use std::collections::{BTreeSet, HashSet};

use advent_of_code::{get_input, solve};

fn main() {
    let input = get_input(4);
    solve(&input, part_one, part_two);
}

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '@' {
                let mut count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0
                            && nx < grid.len() as isize
                            && ny >= 0
                            && ny < grid[nx as usize].len() as isize
                            && grid[nx as usize][ny as usize] == '@'
                        {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    sum += 1;
                }
            }
        }
    }

    sum
}

fn part_two(input: &str) -> usize {
    let mut paper_rolls: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '@' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect();

    fn get_neighbors(
        pos: (isize, isize),
        paper_rolls: &HashSet<(isize, isize)>,
    ) -> HashSet<(isize, isize)> {
        let (x, y) = pos;
        HashSet::from([
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x + 1, y + 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
        ])
        .intersection(paper_rolls)
        .copied()
        .collect()
    }

    let mut sum = 0;
    let mut queue: BTreeSet<(isize, isize)> = paper_rolls.iter().copied().collect();
    while let Some(current) = queue.pop_first() {
        let neighbors = get_neighbors(current, &paper_rolls);
        if neighbors.len() < 4 {
            sum += 1;
            paper_rolls.remove(&current);
            queue.extend(neighbors);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 43);
    }
}
