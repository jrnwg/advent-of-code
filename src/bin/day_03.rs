use advent_of_code::{get_input, solve};
use std::cmp::Ordering;

fn main() {
    let input = get_input(3);
    solve(&input, part_one, part_two);
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (first_idx, first_digit) = line[..line.len() - 1]
                .chars()
                .enumerate()
                .map(|(idx, c)| (idx, c.to_digit(10).unwrap()))
                .max_by(|(idx_x, x), (idx_y, y)| match x.cmp(y) {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Less => Ordering::Less,
                    Ordering::Equal => idx_y.cmp(idx_x),
                })
                .unwrap();

            let second_digit = line[first_idx + 1..]
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .max()
                .unwrap();

            10 * first_digit + second_digit
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    let window_size = 12;
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let digits = line
                .chars()
                .map(|d| d.to_digit(10).unwrap())
                .collect::<Vec<u32>>();

            let mut window: Vec<u32> = digits[..window_size].to_vec();
            for &value in digits.iter().skip(window_size) {
                window.push(value);
                for (idx, v) in window[..window.len() - 1].iter().enumerate() {
                    if v < &window[idx + 1] {
                        window.remove(idx);
                        break;
                    }
                }
                if window.len() > window_size {
                    window.pop();
                }
            }

            window
                .iter()
                .fold(0u64, |acc, &digit| acc * 10 + digit as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_max_by() {
        assert_eq!(part_one("9992"), 99);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 357);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 3121910778619);
    }
}
