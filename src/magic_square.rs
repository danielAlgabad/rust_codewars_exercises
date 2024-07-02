fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let mut square = Vec::new();
    let mut i: i32 = 0;
    let mut j: i32 = n as i32 / 2;
    for _ in 0..n {
        square.push(Vec::from_iter(std::iter::repeat(0).take(n as usize)));
    }

    for number in 1..=n * n {
        println!("{}, {}, {}", number, i, j);
        if square[i as usize][j as usize] == 0 {
            square[i as usize][j as usize] = number;
            i = (((i - 1) % n as i32) + n as i32) % n as i32;
            j = (((j + 1) % n as i32) + n as i32) % n as i32;
        } else {
            i = (((i + 2) % n as i32) + n as i32) % n as i32;
            j = (((j - 1) % n as i32) + n as i32) % n as i32;
            square[i as usize][j as usize] = number;
            i = (((i - 1) % n as i32) + n as i32) % n as i32;
            j = (((j + 1) % n as i32) + n as i32) % n as i32;
        }
    }

    square
}

fn main() {
    let magic_square = magic_square(5);
    println!("{:?}", magic_square);
}
