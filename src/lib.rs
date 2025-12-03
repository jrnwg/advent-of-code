use std::fs;

pub fn get_input(day: u32) -> String {
    let path = format!("inputs/day_{:02}.txt", day);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path))
        .trim()
        .to_string()
}
