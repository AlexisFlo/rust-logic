// Given a number n, find the sum of the first natural numbers.

use std::io::stdin;

fn main() {
    println!("Enter a number");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read input");

    let nat_number: i32 = input.trim().parse().expect("Please enter a valid number");

    let sum = (nat_number * (nat_number + 1)) / 2;

    println!("the sum of the first natural numbers is:  {sum}");
}
