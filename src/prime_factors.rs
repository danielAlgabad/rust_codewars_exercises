use std::collections::BTreeMap;

fn prime_factors(n: i64) -> String {
    let mut number = n;
    let mut prime_factors = BTreeMap::new();

    while number % 2 == 0 {
        number /= 2;
        *prime_factors.entry(2).or_insert(0) += 1;
    }

    let mut i = 3;
    while i * i <= number {
        if number % i == 0 {
            number /= i;
            *prime_factors.entry(i).or_insert(0) += 1;
        } else {
            i += 2;
        }
    }
    if number > 1 {
        *prime_factors.entry(number).or_insert(0) += 1;
    }

    btreemap_to_string(&prime_factors)
}

fn btreemap_to_string(map: &BTreeMap<i64, i64>) -> String {
    map.iter()
        .map(|(&key, &value)| {
            if value > 1 {
                format!("({}**{})", key, value)
            } else {
                format!("({})", key)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let prime_factors = prime_factors(7775460);
    println!("{}", prime_factors);
}
