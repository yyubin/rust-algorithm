use std::io::{self, Write};
use std::collections::VecDeque;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut v: VecDeque<i32> = (1..=n).collect();

    while v.len() > 1 {
        v.pop_front();
        let next = v.pop_front().unwrap();
        v.push_back(next);
    }

    println!("{}", v.front().unwrap());
}