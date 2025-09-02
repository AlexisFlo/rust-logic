fn main() {
    println!("Ingresa tu estatura");
    let esta = 1.70;
    let peso = 60;

    // IMC = kg/m^2
    let imc= peso as f32/(esta* esta); 

    println!("Tu imc es de {imc}");
}
