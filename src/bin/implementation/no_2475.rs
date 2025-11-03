use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let ss: Vec<&str> = s.split_whitespace().collect();

    let mut result = 0;
    for i in 0..ss.len() {
        let tmp: i32 = ss[i].parse().unwrap();
        result += tmp * tmp;
    }
    println!("{}", result%10);
}