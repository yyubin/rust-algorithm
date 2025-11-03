use std::io;

fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();
    io::stdin().read_line(&mut n1).unwrap();
    io::stdin().read_line(&mut n2).unwrap();

    let i1: i32 = n1.trim().parse().unwrap();
    let i2: i32 = n2.trim().parse().unwrap();

    if i1 > 0 {
        if i2 > 0 {
            println!("1")
        } else {
            println!("4")
        }
    } else {
        if i2 > 0 {
            println!("2")
        } else {
            println!("3")
        }
    }

}