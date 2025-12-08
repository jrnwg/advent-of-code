use advent_of_code::get_input;

fn main() {
    let input = get_input(4);
    println!("Part One: {}", part_one(&input));
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn part_one(input: &str) -> u32 {
    let grid = get_grid(input);
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
}
