use std::io::stdin;

// Given a number n, we need to print its table.
fn main() {
    println!("Enter a number");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read input.");

    let mut val1: i8 = input.trim().parse().expect("Please enter a valid number.");
    let mut val2: i8 = 1;

    loop {
        println!("{} x {} = {}", val1, val2, val1 * val2);

        val2 += 1;

        if val2 > 10 {
            break;
        } else if val2 > 10 {
            val1 += 1;
            val2 = 1;
        }
    }
}
