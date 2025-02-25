// Given two numbers a and b, the task is to swap them.
fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    println!("a: {a}, b: {b}");

    let a = a ^ b;
    let b = a ^ b;
    let a = a ^ b;

    println!("a: {a}, b: {b}");
}
