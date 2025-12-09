use advent_of_code::{get_input, solve};

use std::collections::BinaryHeap;
fn main() {
    let input = get_input(9);
    solve(&input, part_one, part_two);
}

type Coord = (usize, usize);

fn init_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<usize>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<Vec<Coord>>()
}

fn part_one(input: &str) -> usize {
    let coords = init_coords(input);
    let mut max: usize = 0;
    for (idx, (x, y)) in coords.iter().enumerate() {
        for (nx, ny) in &coords[idx + 1..] {
            let a = (((*x as isize - *nx as isize).abs() + 1)
                * ((*y as isize - *ny as isize).abs() + 1)) as usize;

            if a > max {
                max = a;
            }
        }
    }
    max
}

fn has_intersection(a: &Coord, b: &Coord, c: &Coord, d: &Coord) -> bool {
    let (ax, ay) = &a;
    let (bx, by) = &b;

    let (x1, y1) = &c;
    let (x2, y2) = &d;

    if ax == bx && y1 == y2 {
        return x1.min(x2) < ax && ax < x1.max(x2) && ay.min(by) < y1 && y1 < ay.max(by);
    } else if ay == by && x1 == x2 {
        return y1.min(y2) < ay && ay < y1.max(y2) && ax.min(bx) < x1 && x1 < ax.max(bx);
    }

    false
}

fn point_in_polygon(point: &Coord, polygon: &[Coord]) -> bool {
    let (px, py) = &point;

    let mut crossings = 0;
    for i in 0..polygon.len() {
        let (x1, y1) = &polygon[i];
        let (x2, y2) = &polygon[(i + 1) % polygon.len()];

        if x1 == x2 && x1 == px && y1.min(y2) <= py && py <= y1.max(y2) {
            return true;
        }
        if y1 == y2 && y1 == py && x1.min(x2) <= px && px <= x1.max(x2) {
            return true;
        }
        if x1 == x2 && x1 < px && y1.min(y2) <= py && py < y1.max(y2) {
            crossings += 1;
        }
    }
    crossings % 2 == 1
}

fn in_polygon(a: &Coord, b: &Coord, polygon: &[Coord]) -> bool {
    let (ax, ay) = &a;
    let (bx, by) = &b;

    if !point_in_polygon(&(*ax, *by), polygon) || !point_in_polygon(&(*bx, *ay), polygon) {
        return false;
    }
    for idx in 0..polygon.len() {
        let c = &polygon[idx];
        let d = &polygon[(idx + 1) % polygon.len()];

        if has_intersection(&(*ax, *ay), &(*ax, *by), c, d)
            || has_intersection(&(*ax, *by), &(*bx, *by), c, d)
            || has_intersection(&(*bx, *by), &(*bx, *ay), c, d)
            || has_intersection(&(*bx, *ay), &(*ax, *ay), c, d)
        {
            return false;
        }
    }

    true
}

fn part_two(input: &str) -> usize {
    let coords = init_coords(input);

    let mut heap = BinaryHeap::new();
    for (idx, (x, y)) in coords.iter().enumerate() {
        for (nx, ny) in &coords[idx + 1..] {
            let a = (((*x as isize - *nx as isize).abs() + 1)
                * ((*y as isize - *ny as isize).abs() + 1)) as usize;

            heap.push((a, (*x, *y), (*nx, *ny)));
        }
    }

    while !heap.is_empty() {
        let (area, a, b) = heap.pop().unwrap();

        if in_polygon(&a, &b, &coords) {
            return area;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 50);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 24);
    }
}
