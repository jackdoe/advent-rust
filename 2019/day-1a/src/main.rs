use std::io::BufRead;
fn add(v: i64) -> i64 {
    if v <= 0 {
        return 0;
    }
    return v + add(v / 3 - 2);
}
fn main() {
    let stdin = std::io::stdin();
    let mut sum: i64 = 0;
    for line in stdin.lock().lines() {
        let value: i64 = line.unwrap().parse().expect("not a number");

        sum += add(value / 3 - 2)
    }
    println!("{}", sum);
}
