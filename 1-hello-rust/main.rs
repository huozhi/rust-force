use std::io::{self, BufRead};

fn main() {
    let line1 = io::stdin().lock().lines().next().unwrap().unwrap();
    let v: Vec<u64> = line1.split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let x = ceil(v[0], v[2]);
    let y = ceil(v[1], v[2]);

    println!("{}", x * y);
}

fn ceil(x: u64, y: u64) -> u64 {
    return (x as f64 / y as f64).ceil() as u64;
}