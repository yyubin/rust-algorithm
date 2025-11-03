use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let nums: Vec<&str> = line.split_whitespace().collect();
    let num1: i32 = nums[0].parse().unwrap();
    let num2: i32 = nums[1].parse().unwrap();

    if num1 > num2 {
        println!(">");
    } else if num1 < num2 {
        println!("<");
    } else {
        println!("==");
    }
}

