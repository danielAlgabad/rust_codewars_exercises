fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let max_inner_size = pyramid.iter().map(|v| v.len()).max().unwrap_or(0);
    let mut slides: Vec<Vec<i64>> = Vec::with_capacity(pyramid.len());

    for inner_vector in pyramid {
        let mut new_inner_vector = vec![-1; max_inner_size];
        new_inner_vector.resize(inner_vector.len(), -1);
        slides.push(new_inner_vector);
    }

    let max_slide = calculate_slide(pyramid.len() - 1, pyramid.len() - 1, pyramid, &mut slides);
    println!("{:?}", slides);
    max_slide
}

fn calculate_slide(i: usize, j: usize, pyramid: &[Vec<u16>], slides: &mut Vec<Vec<i64>>) -> u16 {
    println!("{}, {}", i, j);
    if i == 0 {
        return pyramid[0][0];
    }
    if j == 0 {
        slides[i - 1][j] = calculate_slide(i - 1, j, pyramid, slides) as i64;
        println!("1, {}, {}", i, j);
        println!("1, {}", slides[i - 1][j]);
    } else if slides[i - 1][j - 1] == -1 {
        slides[i - 1][j - 1] = calculate_slide(i - 1, j - 1, pyramid, slides) as i64;
        println!("2, {}, {}", i, j);
        println!("2, {}", slides[i - 1][j - 1]);
    }
    if j < slides[i].len() - 1 {
        if slides[i - 1][j] == -1 {
            slides[i - 1][j] = calculate_slide(i - 1, j, pyramid, slides) as i64;
        }
        println!("3, {}, {}", i, j);
        println!("3, {}", slides[i - 1][j]);
        if j > 0 {
            (slides[i - 1][j].max(slides[i - 1][j - 1]) + pyramid[i][j] as i64) as u16
        } else {
            (slides[i - 1][j] + pyramid[i][j] as i64) as u16
        }
    } else {
        (slides[i - 1][j - 1] + pyramid[i][j] as i64) as u16
    }
}

fn main() {
    let longest_slide = longest_slide_down(&[
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ]);
    println!("{}", longest_slide);
}
