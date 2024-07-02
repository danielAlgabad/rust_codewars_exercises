fn full_cycle(lst: &[usize]) -> bool {
    let mut visited = Vec::new();
    let mut current_position = 0;

    while !visited.contains(&current_position) {
        visited.push(current_position);
        current_position = lst[current_position];
    }

    visited.len() == lst.len()
}

fn main() {
    let full_cycle = full_cycle(&[
        21, 18, 19, 14, 8, 0, 9, 2, 1, 3, 7, 4, 5, 10, 13, 12, 6, 17, 11, 15, 20, 16,
    ]);
    println!("{:?}", full_cycle);
}
