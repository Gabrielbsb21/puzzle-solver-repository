use std::io;

fn main() {
    let amount_to_pay = line_input();
    println!("VALOR A PAGAR: R$ {:.2}", amount_to_pay);
}

fn line_input() -> f64 {
    let mut input_line1 = String::new();
    let mut input_line2 = String::new();
    io::stdin().read_line(&mut input_line1).unwrap();
    io::stdin().read_line(&mut input_line2).unwrap();

    let values_line1: Vec<&str> = input_line1.trim().split_whitespace().collect();
    let values_line2: Vec<&str> = input_line2.trim().split_whitespace().collect();
    let product_unit1: f64 = values_line1[1].parse().unwrap();
    let product_unit2: f64 = values_line2[1].parse().unwrap();
    let product_price1: f64 = values_line1[2].parse().unwrap();
    let product_price2: f64 = values_line2[2].parse().unwrap();

    let result = product_unit1 * product_price1 + product_unit2 * product_price2;
    return result;
}
