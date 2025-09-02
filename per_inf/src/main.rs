use std::io::stdin;

fn main() {
    // add variable that save personal information of the user, like: name, last name, age, city,
    // and the output will be a message with this information
    

    println!("Add your name");
    let mut u_name = String::new();
    stdin().read_line(&mut u_name).expect("Failed to read input");
    println!("Add your last name");
    let mut last_name = String::new();
    stdin().read_line(&mut last_name).expect("Failed to read input");
    println!("Add your age");
    let mut u_age = String::new();
    stdin().read_line(&mut u_age).expect("Failed to read input");
    // let age: u32 = u_age.trim().parse().expect("Please enter a valid number");
    println!("Add your city");
    let mut city = String::new();
    stdin().read_line(&mut city).expect("Failed to read input");

    println!("Hello, my name is {u_name} and my last name is {last_name}, I'm {u_age} and I live in {city}");
}
