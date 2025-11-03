use std::io;

fn main() {
    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();
    let nums: Vec<&str> = l.split_whitespace().collect();
    let a: f64 = nums[0].parse().unwrap();
    let b: f64 = nums[1].parse().unwrap();

    println!("{}", a/b);
}