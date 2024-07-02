fn next_bigger_number(n: u64) -> Option<u64> {
    let num_str = n.to_string();
    let mut char_vec: Vec<char> = num_str.chars().collect();

    find_next_bigger(&mut char_vec)
}

fn find_next_bigger(chars: &mut Vec<char>) -> Option<u64> {
    let len = chars.len();

    let mut i = len - 1;
    while i > 0 && chars[i - 1] >= chars[i] {
        i -= 1;
    }

    if i == 0 {
        return None;
    }

    let mut j = len - 1;
    while chars[j] <= chars[i - 1] {
        j -= 1;
    }

    chars.swap(i - 1, j);

    chars[i..].reverse();
    println!("{:?}", chars);

    if chars[0] == '0' {
        return None;
    }

    Some(
        chars
            .clone()
            .into_iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
    )
}

fn main() {
    let next_bigger_number = next_bigger_number(513);
    println!("{:?}", next_bigger_number);
}
