use advent_of_code::get_input;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

const MAX_PAIRS: usize = 1000;

fn main() {
    let input = get_input(8);
    println!("Part One: {}", part_one(&input, MAX_PAIRS));
    println!("Part Two: {}", part_two(&input, MAX_PAIRS));
}

type Coord = (usize, usize, usize);

fn init_heap(input: &str) -> BinaryHeap<Reverse<(i64, (Coord, Coord))>> {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.parse::<usize>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect::<Vec<Coord>>();

    let mut heap = BinaryHeap::new();
    for (idx, (x, y, z)) in coords.iter().enumerate() {
        for (nx, ny, nz) in &coords[idx + 1..] {
            let d: i64 = (*x as i64 - *nx as i64).pow(2)
                + (*y as i64 - *ny as i64).pow(2)
                + (*z as i64 - *nz as i64).pow(2);
            heap.push(Reverse((d, ((*x, *y, *z), (*nx, *ny, *nz)))));
        }
    }
    heap
}

fn merge_circuits(
    idx_cache: &mut HashMap<Coord, usize>,
    circuits: &mut Vec<HashSet<Coord>>,
    p1: Coord,
    p2: Coord,
) -> usize {
    let mut found_idxs: HashSet<usize> = HashSet::new();

    if let Some(&idx) = idx_cache.get(&p1) {
        found_idxs.insert(idx);
    }
    if let Some(&idx) = idx_cache.get(&p2) {
        found_idxs.insert(idx);
    }

    let idx = match found_idxs.len() {
        0 => {
            let new_circuit = HashSet::new();
            circuits.push(new_circuit);
            circuits.len() - 1
        }
        1 => *found_idxs.iter().next().unwrap(),
        2 => {
            let idx1 = *found_idxs.iter().next().unwrap();
            let idx2 = *found_idxs.iter().nth(1).unwrap();

            let to_merge = std::mem::take(&mut circuits[idx2]);
            for coord in to_merge {
                idx_cache.insert(coord, idx1);
                circuits[idx1].insert(coord);
            }

            idx1
        }
        _ => panic!("More than two circuits found for points"),
    };

    circuits[idx].insert(p1);
    circuits[idx].insert(p2);

    idx_cache.insert(p1, idx);
    idx_cache.insert(p2, idx);
    idx
}

fn part_one(input: &str, max_pairs: usize) -> usize {
    let mut heap = init_heap(input);

    let mut idx_cache: HashMap<Coord, usize> = HashMap::new();
    let mut circuits: Vec<HashSet<Coord>> = Vec::new();
    let mut count = 0;
    while !heap.is_empty() && count < max_pairs {
        count += 1;

        let Reverse((_, (p1, p2))) = heap.pop().unwrap();
        merge_circuits(&mut idx_cache, &mut circuits, p1, p2);
    }

    let mut sizes: Vec<usize> = circuits
        .iter()
        .map(|c| c.len())
        .filter(|&len| len > 0)
        .collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn part_two(input: &str, max_pairs: usize) -> usize {
    let mut heap = init_heap(input);

    let mut idx_cache: HashMap<Coord, usize> = HashMap::new();
    let mut circuits: Vec<HashSet<Coord>> = Vec::new();
    while !heap.is_empty() {
        let Reverse((_, (p1, p2))) = heap.pop().unwrap();
        let idx = merge_circuits(&mut idx_cache, &mut circuits, p1, p2);

        if circuits[idx].len() >= max_pairs {
            return p1.0 * p2.0;
        }
    }

    unreachable!()
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
        assert_eq!(part_one(TEST_INPUT, 10), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT, 20), 25272);
    }
}
