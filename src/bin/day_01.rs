use advent_of_code::get_input;

fn main() {
    let input = get_input(1);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for rotation in input.split_whitespace() {
        let (cmd_str, val_str) = rotation.split_at(1);
        let command = cmd_str.chars().next().unwrap();
        let value: i32 = val_str.parse().unwrap();

        dial = match command {
            'L' => (dial - value).rem_euclid(100),
            'R' => (dial + value).rem_euclid(100),
            _ => dial,
        };

        if dial == 0 {
            password += 1;
        }
    }

    password
}

fn part_two(input: &str) -> i32 {
    let mut dial = 50;
    let mut password = 0;

    for rotation in input.split_whitespace() {
        let (cmd_str, val_str) = rotation.split_at(1);
        let command = cmd_str.chars().next().unwrap();
        let value: i32 = val_str.parse().unwrap();

        let norm_value = value % 100;
        password += value / 100;
        password += match command {
            'L' => (dial > 0 && dial - norm_value <= 0) as i32,
            'R' => (dial + norm_value >= 100) as i32,
            _ => 0,
        };

        dial = match command {
            'L' => (dial - value).rem_euclid(100),
            'R' => (dial + value).rem_euclid(100),
            _ => dial,
        };
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 6);
    }
}
