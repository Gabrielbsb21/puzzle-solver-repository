use std::io;

fn main() {
    let mut employer_number = String::new();
    let mut worked_hours = String::new();
    let mut salary_per_hour = String::new();

    io::stdin().read_line(&mut employer_number).unwrap();
    io::stdin().read_line(&mut worked_hours).unwrap();
    io::stdin().read_line(&mut salary_per_hour).unwrap();

    let employer_number: i32 = employer_number.trim().parse().unwrap();
    let worked_hours: i32 = worked_hours.trim().parse().unwrap();
    let salary_per_hour: f64 = salary_per_hour.trim().parse().unwrap();

    let result: f64 = worked_hours as f64 * salary_per_hour;

    println!("NUMBER = {}", employer_number);
    println!("SALARY = U$ {:.2}", result);
}