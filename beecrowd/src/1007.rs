use std::io;

fn main() {
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut number_three = String::new();
    let mut number_four = String::new();

    io::stdin().read_line(&mut number_one).unwrap();
    io::stdin().read_line(&mut number_two).unwrap();
    io::stdin().read_line(&mut number_three).unwrap();
    io::stdin().read_line(&mut number_four).unwrap();

    let number_one: i32 = number_one.trim().parse().unwrap();
    let number_two: i32 = number_two.trim().parse().unwrap();
    let number_three: i32 = number_three.trim().parse().unwrap();
    let number_four: i32 = number_four.trim().parse().unwrap();

    let result = (number_one * number_two) - (number_three * number_four);

    println!("DIFERENCA = {}", result);
}