use std::io;

fn main() {
    let mut employer_name = String::new();
    let mut employer_salary = String::new();
    let mut employer_total_sold = String::new();

    io::stdin().read_line(&mut employer_name).unwrap();
    io::stdin().read_line(&mut employer_salary).unwrap();
    io::stdin().read_line(&mut employer_total_sold).unwrap();

    let employer_salary: f64 = employer_salary.trim().parse().unwrap();
    let employer_total_sold: f64 = employer_total_sold.trim().parse().unwrap();

    let result: f64 = employer_salary + employer_total_sold * 0.15;

    println!("TOTAL = R$ {:.2}", result);

}