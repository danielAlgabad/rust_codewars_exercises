fn count_bits(n: i64) -> u32 {
    let mut count = 0;
    let mut quotient = n;

    while quotient > 0 {
        if quotient % 2 == 1 {
            count += 1;
        }
        quotient = quotient / 2;
    }
    //n.count_ones();

    count
}

fn main() {
    let count_bits = count_bits(7);
    println!("{:?}", count_bits);
}
