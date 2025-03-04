use std::io::stdin;

fn main() {

    let mut input = String::new();
    let mut input_1 = String::new();

    println!("Enter the first number");
    stdin().read_line(&mut input).expect("Failed to read input");

    let number1: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("Now the second number");
    stdin().read_line(&mut input_1).expect("Failed to read input");

    let number2: i32 = input_1.trim().parse().expect("Please enter a valid number");

    if number1 > number2 {
        println!("{number1} is greather than {number2}")
    } else if number1 < number2 {
        println!("{number1} is less than {number2}")
    } else {
        println!("both numbers are equals")
    }
}
