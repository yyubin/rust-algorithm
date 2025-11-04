mod no_10828_re;

use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let mut stack: Vec<i32> = Vec::new();

    let mut output = String::new();

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let arr: Vec<&str> = s.split_whitespace().collect();
        match arr[0] {
            "push" => {
                let num: i32 = arr[1].parse().unwrap();
                stack.push(num);
            }
            "pop" => {
                if stack.len() <= 0 {
                    output.push_str("-1\n");
                } else {
                    let i = stack.pop().unwrap();
                    output.push_str(&format!("{i}\n"));
                }
            }
            "size" => {
                let size = stack.len();
                output.push_str(&format!("{size}\n"));
            }
            "empty" => {
                let size = stack.len();
                if size > 0 {
                    output.push_str("0\n");
                } else {
                    output.push_str("1\n");
                }
            }
            "top" => {
                let size = stack.len();
                if size > 0 {
                    let t = stack[size-1];
                    output.push_str(&format!("{t}\n"));
                } else {
                    output.push_str("-1\n");
                }
            }
            _ => {}
        }
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}