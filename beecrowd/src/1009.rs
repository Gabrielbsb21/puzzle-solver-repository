use stdin::io;

fn main() {
    let mut employer_name = String::new();
    let mut employer_salary = String::new();
    let mut employer_total_sold = String::new();

    io::stdin().read_line(&mut employer_name).unwrap();
    io::stdin().read_line(&mut employer_salary).unwrap();
    io::stdin().read_line(&mut employer_total_sold).unwrap();

}