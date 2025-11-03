mod ab_1001;

use std::io;

fn main() {
    let mut line = String::new();

    io::stdin().read_line(&mut line).unwrap();

    let nums: Vec<&str> = line.split_whitespace().collect();
    let num_a: i32 = nums[0].parse().unwrap();
    let num_b: i32 = nums[1].parse().unwrap();

    println!("{}", num_a + num_b);
}