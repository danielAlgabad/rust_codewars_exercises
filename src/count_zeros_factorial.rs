fn zeros(n: u64) -> u64 {
    let mut count = 0;
    let mut divisor = 5;

    while n >= divisor {
        count += n / divisor;
        divisor *= 5;
    }
    count
}

fn main() {
    let zeros = zeros(30);
    println!("{}", zeros);
}
