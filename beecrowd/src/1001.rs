use std::io;

fn main() {
    let mut aux_one = String::new();
    let mut aux_two = String::new();

    io::stdin().read_line(&mut aux_one).unwrap();
    io::stdin().read_line(&mut aux_two).unwrap();

    let number_one: i32 = aux_one.trim().parse().unwrap();
    let number_two: i32 = aux_two.trim().parse().unwrap();

    println!("X = {}", number_one + number_two);
}