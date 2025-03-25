
fn closest_number(n: i32, m: i32) -> i32 {
    // find the quotient
    let q = n / m;

    // 1st possible closest number
    let n1 = m * q;

    // 2nd possible closest number
    let n2 = if (n * m) > 0 { m * (q + 1) } else { m * (q - 1)};

    if (n - n1).abs() < (n - n2).abs() {
        n1
    } else {
        n2
    }
}

#[allow(unused_variables)]
fn main() {
    let n = 13;
    let m = 4;

    println!("Número más cercano entre 13 y 4: {}", closest_number(13, 4));
}
