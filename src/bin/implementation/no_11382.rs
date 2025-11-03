use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let nums: Vec<&str> = line.split_whitespace().collect();
    let num1: i64 = nums[0].parse().unwrap();
    let num2: i64 = nums[1].parse().unwrap();
    let num3: i64 = nums[2].parse().unwrap();

    println!("{}", num1 + num2 + num3);
}