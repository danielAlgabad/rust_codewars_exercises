fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let (mut c1, mut c2, mut c3) = (Vec::new(), Vec::new(), Vec::new());
    let (mut d1, mut d2) = (Vec::new(), Vec::new());
    let mut has_zero = false;

    for (i, row) in board.iter().enumerate() {
        if row.iter().all(|x| *x == 1) {
            return 1;
        } else if row.iter().all(|x| *x == 2) {
            return 2;
        } else {
            c1.push(row[0]);
            c2.push(row[1]);
            c3.push(row[2]);
            d1.push(row[i]);
            d2.push(row[2 - i]);
            if !has_zero {
                for elem in row.iter() {
                    if *elem == 0 {
                        has_zero = true;
                        break;
                    }
                }
            }
        }
    }
    if c1.iter().all(|x| *x == 1)
        || c2.iter().all(|x| *x == 1)
        || c3.iter().all(|x| *x == 1)
        || d1.iter().all(|x| *x == 1)
        || d2.iter().all(|x| *x == 1)
    {
        return 1;
    } else if c1.iter().all(|x| *x == 2)
        || c2.iter().all(|x| *x == 2)
        || c3.iter().all(|x| *x == 2)
        || d1.iter().all(|x| *x == 2)
        || d2.iter().all(|x| *x == 2)
    {
        return 2;
    } else if has_zero == true {
        return -1;
    }
    0
}

fn main() {
    let is_solved = is_solved(&[&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]]);
    println!("{}", is_solved);
}
