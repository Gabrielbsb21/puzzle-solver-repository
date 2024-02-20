use std::io;

fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();

    io::stdin().read_line(&mut number_one).unwrap();
    io::stdin().read_line(&mut number_two).unwrap();

    let number_one: i32 = number_one.trim().parse().unwrap();
    let number_two: i32 = number_two.trim().parse().unwrap();

    println!("PROD = {}", number_one * number_two);
}