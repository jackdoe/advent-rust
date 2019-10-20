use std::io::BufRead;
fn main() {
    let stdin = std::io::stdin();

    let mut count2: i64 = 0;
    let mut count3: i64 = 0;
    let mut codes = vec![];
    for line in stdin.lock().lines() {
        let mut count_per_char: [i64; 26] = [0; 26];
        let l = line.unwrap();

        for c in l.chars() {
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

        codes.push(l);
    }

    println!("check sum: {}", count2 * count3);
    'FOUND: for code_a in codes.iter() {
        for code_b in codes.iter() {
            let (different, common) = diff(code_a, code_b);
            if different == 1 {
                println!("fabric code: {}", common);
                break 'FOUND;
            }
        }
    }
}

fn diff(a: &String, b: &String) -> (i32, String) {
    let ba = a.as_bytes();
    let bb = b.as_bytes();

    if ba.len() != bb.len() {
        return (ba.len() as i32 + bb.len() as i32, a.to_string());
    }

    let mut different: i32 = 0;
    let mut common: String = String::new();

    for i in 0..ba.len() {
        if ba[i] != bb[i] {
            different += 1;
        } else {
            common.push(ba[i] as char)
        }
    }

    return (different, common);
}
