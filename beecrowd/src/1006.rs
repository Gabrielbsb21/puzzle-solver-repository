use std::io;

fn main() {
    let mut aux_one = String::new();
    let mut aux_two = String::new();
    let mut aux_three = String::new();

    io::stdin().read_line(&mut aux_one).unwrap();
    io::stdin().read_line(&mut aux_two).unwrap();
    io::stdin().read_line(&mut aux_three).unwrap();

    let number_one: f64 = aux_one.trim().parse().unwrap();
    let number_two: f64 = aux_two.trim().parse().unwrap();
    let number_three: f64 = aux_three.trim().parse().unwrap();

    let media: f64 = (number_one * 2.0 + number_two * 3.0 + number_three * 5.0) / 10.0;

    println!("MEDIA = {:.1}", media);
}