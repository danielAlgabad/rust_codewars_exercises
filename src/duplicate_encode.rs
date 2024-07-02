fn duplicate_encode(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .map(|x| {
            if word.to_lowercase().matches(x).count() > 1 {
                ")"
            } else {
                "("
            }
        })
        .collect()
}

fn main() {
    let duplicate_encode = duplicate_encode("Success");
    println!("{:?}", duplicate_encode);
}
