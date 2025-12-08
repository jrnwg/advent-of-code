use advent_of_code::get_input;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

const MAX_PAIRS: usize = 1000;

fn main() {
    let input = get_input(8);
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

fn part_one(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i64>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64, i64)>>();

    let mut heap = BinaryHeap::new();
    for (idx, (x, y, z)) in coords.iter().enumerate() {
        for (nx, ny, nz) in &coords[idx + 1..] {
            let d = (x - nx).pow(2) + (y - ny).pow(2) + (z - nz).pow(2);
            heap.push(Reverse((d, ((*x, *y, *z), (*nx, *ny, *nz)))));
        }
    }

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> = Vec::new();
    let mut count = 0;
    while !heap.is_empty() && count < MAX_PAIRS {
        count += 1;

        let Reverse((_, (p1, p2))) = heap.pop().unwrap();
        let mut found_idxs: Vec<usize> = Vec::new();
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&p1) || circuit.contains(&p2) {
                found_idxs.push(idx);
            }
        }

        match found_idxs.len() {
            0 => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(p1);
                new_circuit.insert(p2);
                circuits.push(new_circuit);
            }
            1 => {
                let circuit = &mut circuits[found_idxs[0]];
                circuit.insert(p1);
                circuit.insert(p2);
            }
            _ => {
                let to_merge = circuits.swap_remove(found_idxs[1]);
                circuits[found_idxs[0]].extend(to_merge);
                circuits[found_idxs[0]].insert(p1);
                circuits[found_idxs[0]].insert(p2);
            }
        }
    }

    let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    sizes.sort_by_key(|&len| std::cmp::Reverse(len));
    sizes.iter().take(3).product::<usize>()
}

fn part_two(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<i64>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<(i64, i64, i64)>>();

    let mut heap = BinaryHeap::new();
    for (idx, (x, y, z)) in coords.iter().enumerate() {
        for (nx, ny, nz) in &coords[idx + 1..] {
            let d = (x - nx).pow(2) + (y - ny).pow(2) + (z - nz).pow(2);
            heap.push(Reverse((d, ((*x, *y, *z), (*nx, *ny, *nz)))));
        }
    }

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> = Vec::new();
    while !heap.is_empty() {
        let Reverse((_, (p1, p2))) = heap.pop().unwrap();
        let mut found_idxs: Vec<usize> = Vec::new();
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&p1) || circuit.contains(&p2) {
                found_idxs.push(idx);
            }
        }

        match found_idxs.len() {
            0 => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(p1);
                new_circuit.insert(p2);
                circuits.push(new_circuit);
            }
            1 => {
                let circuit = &mut circuits[found_idxs[0]];
                circuit.insert(p1);
                circuit.insert(p2);
            }
            _ => {
                let to_merge = circuits.swap_remove(found_idxs[1]);
                circuits[found_idxs[0]].extend(to_merge);
                circuits[found_idxs[0]].insert(p1);
                circuits[found_idxs[0]].insert(p2);
            }
        }

        let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
        sizes.sort_by_key(|&len| std::cmp::Reverse(len));

        if *sizes.first().unwrap() == 1000 {
            return p1.0 as usize * p2.0 as usize;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 25272);
    }
}
