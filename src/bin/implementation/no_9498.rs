use std::io;

fn main() {
    let mut native = String::new();
    io::stdin().read_line(&mut native).unwrap();

    let score: i32 = native.trim().parse().unwrap();

    if score >= 90 {
        println!("A");
    } else if score >= 80 {
        println!("B");
    } else if score >= 70 {
        println!("C");
    } else if score >= 60 {
        println!("D");
    } else {
        println!("F");
    }
}