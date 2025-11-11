use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, mut k) = (iter.next().unwrap(), iter.next().unwrap());

    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let coin: i32 = input.trim().parse().unwrap();
        v.push(coin);
    }

    let mut res = 0;
    for i in v.into_iter().rev() {
        let m = k/i;
        if m > 0 {
            res += k/i;
            k %= i;
        }
    }

    println!("{}", res);
}