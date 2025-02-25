// Given two numbers a and b, the task is to swap them.
fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    println!("a: {a}, b: {b}");

    let temp = a;
    let a = b;
    let b = temp;

    println!("a: {a}, b: {b}");
}
