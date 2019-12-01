use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();
    let mut sum: i64 = 0;
    for line in stdin.lock().lines() {
        let value: i64 = line.unwrap().parse().expect("not a number");
        sum += value / 3 - 2
    }
    println!("{}", sum);
}
