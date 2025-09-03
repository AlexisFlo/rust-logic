fn main() {
    let num = "123";
    println!("{num}");
    let num1: u32 = num.parse().expect("Not a number");
    let sum = num1 + 77;
    let sumc: String = sum.to_string(); 
    println!("{sumc}");
}
