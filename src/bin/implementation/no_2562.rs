use std::io;

fn main() {
    let mut max = 0;
    let mut idx = 0;
    for i in 0..9 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n: i32 = s.trim().parse().unwrap();

        if n > max {
            max = n;
            idx = i+1;
        }
    }
    println!("{}", max);
    println!("{}", idx);
}