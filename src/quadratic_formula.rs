fn quadratic_formula(y1: i32, y2: i32, y3: i32) -> (String, i32, i32) {
    let b = y2 - y1 - 3;
    let c = y1 - 1 - b;
    println!("{}, {}", b, c);
    let mut formula = String::from("x^2");

    if b != 0 && b != 1 && b != -1 {
        if b > 0 {
            formula.push_str(format!("+{}x", b).as_str());
        } else {
            formula.push_str(format!("{}x", b).as_str());
        }
    } else if b == 1 || b == -1 {
        if b > 0 {
            formula.push_str("+x");
        } else {
            formula.push_str("-x");
        }
    }

    if c != 0 {
        if c > 0 {
            formula.push_str(format!("+{}", c).as_str());
        } else {
            formula.push_str(format!("{}", c).as_str());
        }
    }
    let next1 = 4 * 4 + 4 * b + c;
    let next2 = 5 * 5 + 5 * b + c;

    (formula, next1, next2)
}

fn main() {
    let quadratic_formula = quadratic_formula(0, 2, 6);
    println!("{:?}", quadratic_formula);
}
