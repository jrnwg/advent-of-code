use std::fmt::Display;
use std::fs;
use std::time::{Duration, Instant};

pub fn get_input(day: u32) -> String {
    let path = format!("inputs/day_{:02}.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path))
        .trim()
        .to_string()
}

pub fn time_it<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    (result, elapsed)
}

pub fn solve_one<T1>(input: &str, part_one: fn(&str) -> T1)
where
    T1: Display,
{
    let (p1, t1) = time_it(|| part_one(input));
    println!("Part 1: {} ({:.2?})", p1, t1);
}

pub fn solve<T1, T2>(input: &str, part_one: fn(&str) -> T1, part_two: fn(&str) -> T2)
where
    T1: Display,
    T2: Display,
{
    let (p1, t1) = time_it(|| part_one(input));
    println!("Part 1: {} ({:.2?})", p1, t1);

    let (p2, t2) = time_it(|| part_two(input));
    println!("Part 2: {} ({:.2?})", p2, t2);
}
