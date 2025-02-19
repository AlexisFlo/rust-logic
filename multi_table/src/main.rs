// Given a number n, we need to print its table.
fn main() {
    let mut val1: i8 = 1;
    let mut val2: i8 = 1;

    loop {
        println!("{} x {} = {}", val1, val2, val1 * val2);

        val2 += 1;

        if val1 == 10 && val2 > 10 {
            break;
        } else if val2 > 10 {
            val1 += 1;
            val2 = 1;
        }
    }
}
