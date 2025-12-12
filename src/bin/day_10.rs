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

#[allow(unused)]
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

fn gaussian_elimination_gf2(target: &[bool], buttons: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let (rows, cols) = (target.len(), buttons.len() + 1);
    let mut matrix = vec![vec![false; cols]; rows];

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate().take(cols - 1) {
            *cell = buttons[j][i];
        }
        row[cols - 1] = target[i];
    }

    let mut current_row = 0;
    for col in 0..cols - 1 {
        if let Some((r, _)) = matrix
            .iter()
            .enumerate()
            .skip(current_row)
            .find(|(_, row)| row[col])
        {
            matrix.swap(current_row, r);
            let pivot_data: Vec<_> = matrix[current_row][col..].to_vec();
            for (r, row) in matrix.iter_mut().enumerate() {
                if r != current_row && row[col] {
                    for (i, &val) in pivot_data.iter().enumerate() {
                        row[col + i] ^= val;
                    }
                }
            }
            current_row += 1;
        }
    }

    matrix
}

fn min_presses_by_back_substitution(matrix: Vec<Vec<bool>>) -> usize {
    let n = matrix[0].len() - 1;
    let pivots: Vec<_> = matrix
        .iter()
        .filter_map(|r| r.iter().take(n).position(|&x| x))
        .collect();
    let free: Vec<_> = (0..n).filter(|c| !pivots.contains(c)).collect();

    (0..(1 << free.len()))
        .map(|mask| {
            let mut sol = vec![false; n];
            free.iter()
                .enumerate()
                .for_each(|(i, &c)| sol[c] = (mask >> i) & 1 == 1);

            for r in (0..matrix.len()).rev() {
                if let Some(pivot) = matrix[r].iter().take(n).position(|&x| x) {
                    sol[pivot] = matrix[r][n]
                        ^ (0..n)
                            .filter(|&c| c != pivot && matrix[r][c])
                            .fold(false, |acc, c| acc ^ sol[c]);
                }
            }

            sol.iter().filter(|&&x| x).count()
        })
        .min()
        .unwrap_or(0)
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (target, buttons) = parse_line(line);

            // dfs(&target, &vec![false; target.len()], &buttons, &0).unwrap_or(0)

            let echelon = gaussian_elimination_gf2(&target, &buttons);
            min_presses_by_back_substitution(echelon)
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
