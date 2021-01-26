use std::cmp::max;
use std::io;

fn bigint_add(a: &str, b: &str) -> String {
    let max_len = max(a.len(), b.len());
    let mut carry = 0;
    let mut vec: Vec<char> = Vec::new();
    for idx in 0..max_len {
        let a_curr_bit = if idx < a.len() {
            a.as_bytes()[a.len() - idx - 1] - b'0'
        } else {
            0
        };
        let b_curr_bit = if idx < b.len() {
            b.as_bytes()[b.len() - idx - 1] - b'0'
        } else {
            0
        };
        let mut result = a_curr_bit + b_curr_bit + carry;
        if result >= 10 {
            result = result - 10;
            carry = 1;
        } else {
            carry = 0;
        }
        vec.push((b'0' + result) as char);
    }
    if carry > 0 {
        vec.push((b'0' + carry) as char)
    }
    vec.reverse();
    vec.into_iter().collect()
}

fn main() {
    let mut numbers = Vec::new();
    for _ in 0..3 {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        numbers.push(buff.clone());
    }

    for idx_left in 0..3 {
        for idx_right in 0..3 {
            for idx_target in 0..3 {
                let curr_sum = bigint_add(
                    numbers[idx_left].clone().trim(),
                    numbers[idx_right].clone().trim(),
                );
                if numbers[idx_target].trim().eq(&curr_sum) {
                    print!("YES");
                    return;
                }
            }
        }
    }
    print!("NO");
}
