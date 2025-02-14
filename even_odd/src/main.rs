use std::io::stdin;

fn is_even_or_odd(number: i32) {
    if number % 2 == 0 {
        println!("{number} is Even"); 
    } else {
        println!("{number} is Odd");
    }
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed to read input.");

    let number: i32 = input.trim().parse().expect("Please enter a valid number.");
    
    is_even_or_odd(number);

}
