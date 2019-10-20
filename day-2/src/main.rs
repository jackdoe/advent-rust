use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();

    let mut count2: i64 = 0;
    let mut count3: i64 = 0;

    for line in stdin.lock().lines() {
        let mut count_per_char: [i64; 26] = [0; 26];
        for c in line.unwrap().chars() {
            count_per_char[(c as usize) - 97] += 1; // char - 'a'
        }

        let mut counted2: bool = false;
        let mut counted3: bool = false;
        for n in count_per_char.iter() {
            if n == &2 && !counted2 {
                count2 += 1;
                counted2 = true;
            } else if n == &3 && !counted3 {
                count3 += 1;
                counted3 = true;
            }
        }
    }

    println!("{}", count2 * count3)
}
