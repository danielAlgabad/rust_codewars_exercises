use std::collections::HashSet;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn is_pangram(s: &str) -> bool {
    let mut seen = HashSet::new();

    s.to_lowercase()
        .chars()
        .filter(|&x| seen.insert(x) && ASCII_LOWER.contains(&x))
        .count()
        == ASCII_LOWER.len()
}

fn main() {
    let is_pangram = is_pangram("Aacdefghijklmnopqrstuvwxyz");
    println!("{:?}", is_pangram);
}
