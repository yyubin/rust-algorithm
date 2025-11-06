use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let nn: i32 = n.trim().parse().unwrap();

    let mut res: i32 = 1;
    let mut now: i32 = 1;
    loop {
        if now >= nn {
            break;
        }
        now += 6*res;
        res += 1;
    }
    println!("{}", res);
}