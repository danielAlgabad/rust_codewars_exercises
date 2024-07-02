fn multiply(a: &str, b: &str) -> String {
    if a == "0" || b == "0" {
        return "0".to_string();
    }
    let mut result = vec![0; a.len() + b.len()];

    for (i, digit1) in a.chars().rev().enumerate() {
        let digit1_value = digit1.to_digit(10).unwrap() as u64;
        let mut carry = 0;

        for (j, digit2) in b.chars().rev().enumerate() {
            let digit2_value = digit2.to_digit(10).unwrap() as u64;
            let product = digit1_value * digit2_value + carry + result[i + j];
            result[i + j] = product % 10;
            carry = product / 10;
        }

        if carry > 0 {
            result[i + b.len()] = carry;
        }
    }

    let mut result_str = String::new();
    for digit in result.iter().rev() {
        result_str.push_str(&digit.to_string());
    }

    result_str.trim_start_matches('0').to_string()
}

fn main() {
    let product = multiply("58608473622772837728372827", "7586374672263726736374");
    println!("{}", product);
}
