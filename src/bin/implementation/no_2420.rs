use std::io;

fn main() {
    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();

    let nums: Vec<&str> = l.split_whitespace().collect();
    let n1: i32 = nums[0].parse().unwrap();
    let n2: i32 = nums[1].parse().unwrap();

    if n1 > n2 {
        println!("{}", n1 - n2);
    } else {
        println!("{}", n2 - n1);
    }
}