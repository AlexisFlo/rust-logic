
fn closest_number(n: i32, m: i32) -> i32 {
    // find the quotient
    let mut closest: i32 = 0;
    let mut min_difference = i32::MAX;
    
    println!("Looking for multiples of {m} near {n}");
    println!("Rango de búsqueda: {} a {}", n - m.abs(), n + m.abs());

    // Check numbers around n
    for i in (n - m.abs())..=(n + m.abs()) {
        if i % m == 0 {
            let difference = (n - i).abs();

            if difference < min_difference || (difference == min_difference && i.abs() > closest.abs()) {
                closest = i;
                min_difference = difference;
            }
        }
    }
    closest
}

fn main() {
    println!("Números cercanos");
    println!("Número más cercano a 13 múltiplo de 4: {}", closest_number(13, 4));
}
