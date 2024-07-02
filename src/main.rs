use std::collections::HashMap;

fn list_position(word: &str) -> u128 {
    let sorted_word: String = sort_word(word);
    println!("{}", sorted_word);
    let mut letter_position;
    let mut position = 0;
    let mut i = 0;
    let mut letter_count: HashMap<char, usize> = HashMap::new();

    for letter in word.chars() {
        *letter_count.entry(letter).or_insert(0) += 1;
    }

    for letter in word.chars() {
        letter_position = sorted_word.find(letter).unwrap();
        position += (sorted_word
            .chars()
            .into_iter()
            .filter(|x| *x < letter && *letter_count.get(&letter).unwrap() as i64 > 0)
            .count() as i64)
            * factorial(word.len() as i64 - i - 1) as i64;
        println!(
            "{}, {}, {}, {}",
            letter,
            sorted_word
                .chars()
                .into_iter()
                .filter(|x| *x < letter && *letter_count.get(&letter).unwrap() as i64 > 0)
                .count() as i64,
            letter_position,
            position
        );
        i += 1;
        *letter_count.get_mut(&letter).unwrap() -= 1;
    }

    (position + 1) as u128
}

fn factorial(n: i64) -> i64 {
    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }
    println!("{}, {}", n, result);
    result
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();

    chars.sort_unstable();
    chars.iter().collect()
}

fn main() {
    let list_position = list_position("ABAB");
    println!("{:?}", list_position);
}
