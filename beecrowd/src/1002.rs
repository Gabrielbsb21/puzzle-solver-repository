use std::io;

fn main() {
    let pi = 3.14159;
    let mut aux_one = String::new();

    io::stdin().read_line(&mut aux_one).unwrap();
    let circumference = aux_one.trim().parse::<f64>().unwrap();

    println!("A={:.4}", pi * circumference.powi(2));
}