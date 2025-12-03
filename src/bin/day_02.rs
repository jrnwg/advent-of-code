use advent_of_code::get_input;

fn main() {
    let input = get_input(2);
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for range in input.split(',') {
        let range = range.trim();
        let start_end: Vec<&str> = range.split('-').collect();
        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();

        for n in start..=end {
            let n_str = n.to_string();
            let i = n_str.len() / 2;
            let (first, last) = n_str.split_at(i);
            if first == last {
                sum += n;
            }
        }
    }

    sum
}

fn part_two(input: &str) -> u64 {
    let mut sum: u64 = 0;

    for range in input.split(',') {
        let range = range.trim();
        let start_end: Vec<&str> = range.split('-').collect();
        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();

        for n in start..=end {
            let n_str = n.to_string();
            for l in 2..=n_str.len() {
                if n_str.len() % l != 0 {
                    continue;
                }

                let i = n_str.len() / l;
                let pattern = &n_str[0..i];
                if pattern.repeat(l) == n_str {
                    sum += n;
                    break;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4174379265);
    }
}
