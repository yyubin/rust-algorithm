use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut result = String::new();
    for _ in 0..n {
        let mut l = String::new();
        io::stdin().read_line(&mut l).unwrap();
        let nums: Vec<&str> = l.split_whitespace().collect();
        let a: i32 = nums[0].parse().unwrap();
        let b: i32 = nums[1].parse().unwrap();
        result.push_str(&format!("{}\n", a+b));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", result).unwrap();
}