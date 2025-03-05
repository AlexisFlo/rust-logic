use std::io::stdin;

fn main() {

    let number1 = get_number("Enter the first number");

    let number2 = get_number("Enter the sencond number");

    if number1 > number2 {
        println!("{number1} is greather than {number2}")
    } else if number1 < number2 {
        println!("{number1} is less than {number2}")
    } else {
        println!("both numbers are equals")
    }
}

fn get_number(prompt: &str)  -> i32 {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(number) => return number,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
