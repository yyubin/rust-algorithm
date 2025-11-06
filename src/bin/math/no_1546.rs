mod no_10989;

use std::io;

fn main() {
    let mut n = String::new();
    let mut s = String::new();
    io::stdin().read_line(&mut n).unwrap();
    io::stdin().read_line(&mut s).unwrap();

    let res: Vec<&str> = s.split_whitespace().collect();

    let mut max: f64 = 0.0;
    for r in res.iter() {
        let r_= r.parse::<f64>().unwrap();
        if r_ > max {
            max = r_;
        }
    }

    let mut result: f64 = 0.0;
    for r in res.iter() {
        let r_= r.parse::<f64>().unwrap();
        result += r_/max*100.0;
    }

    let n: f64 = n.trim().parse::<f64>().unwrap();
    println!("{}", result/n);
}