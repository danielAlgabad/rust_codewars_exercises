fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    let filtered_array = array
        .into_iter()
        .filter(|x| *x % 2 == 0)
        .cloned()
        .collect::<Vec<i32>>()
        .to_vec();

    filtered_array[filtered_array.len() - number..].to_vec()
}

fn main() {
    let even_numbers = even_numbers(&vec![6, -25, 3, 7, 5, 5, 7, -3, 23], 1);
    println!("{:?}", even_numbers);
}
