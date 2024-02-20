use std::io;

fn main() {
    let mut aux_one = String::new();
    let mut aux_two = String::new();

    io::stdin().read_line(&mut aux_one).unwrap();
    io::stdin().read_line(&mut aux_two).unwrap();

    let number_one: f64 = aux_one.trim().parse().unwrap();
    let number_two: f64 = aux_two.trim().parse().unwrap();

    let media: f64 = (number_one * 3.5 + number_two * 7.5) / 11.0;

    println!("MEDIA = {:.5}", media);
}