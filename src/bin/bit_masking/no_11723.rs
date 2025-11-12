use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let m: i32 = input.trim().parse().unwrap();

    let mut s: u32 = 0;
    let mut out = io::BufWriter::new(io::stdout());

    for _ in 0..m {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let mut iter = line.split_whitespace();
        let cmd = iter.next().unwrap();
        let arg = iter.next().and_then(|x| x.parse::<u32>().ok());

        match cmd {
            "add" => {
                if let Some(x) = arg {
                    s |= 1 << x;
                }
            }
            "remove" => {
                if let Some(x) = arg {
                    s &= !(1 << x);
                }
            }
            "check" => {
                if let Some(x) = arg {
                    let tmp = if s & (1 << x) != 0 { 1 } else { 0 };
                    writeln!(out, "{tmp}").unwrap();
                }
            }
            "toggle" => {
                if let Some(x) = arg {
                    s ^= 1 << x;
                }
            }
            "empty" => {
                s = 0;
            }
            "all" => {
                s = (1 << 21) - 2;
            }
            _ => {}
        }
    }
}