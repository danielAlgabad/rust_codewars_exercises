use std::collections::VecDeque;

fn range_extraction(a: &[i32]) -> String {
    let mut formatted_string = Vec::new();
    let mut aux_string = VecDeque::new();

    for (i, &num) in a.iter().enumerate() {
        if let Some(next_num) = a.get(i + 1) {
            if next_num - num == 1 || num - next_num == 1 {
                aux_string.push_back(num);
            } else {
                if aux_string.len() >= 2 {
                    formatted_string.push(format!("{}-{},", aux_string.pop_front().unwrap(), num));
                } else {
                    if aux_string.len() == 1 {
                        formatted_string.push(format!("{},", aux_string.pop_front().unwrap()));
                    }
                    formatted_string.push(format!("{},", num));
                }
                aux_string.clear();
            }
        } else {
            if aux_string.len() >= 2 {
                formatted_string.push(format!("{}-{}", aux_string.pop_front().unwrap(), num));
                aux_string.clear();
            } else {
                if aux_string.len() == 1 {
                    formatted_string.push(format!("{},", aux_string.pop_front().unwrap()));
                }
                formatted_string.push(format!("{}", num));
            }
            aux_string.clear();
        }
    }
    formatted_string.join("")
}

fn main() {
    let range_extraction = range_extraction(&[
        -6, -3, -2, -1, 0, 1, 3, 4, 5, 7, 8, 9, 10, 11, 14, 15, 17, 18, 19, 20,
    ]);
    println!("{}", range_extraction);
}
