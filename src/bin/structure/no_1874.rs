use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();
    let mut want: Vec<i32> = Vec::new();

    for _ in 0..n {
        let mut tmp = String::new();
        io::stdin().read_line(&mut tmp).unwrap();
        let tmp: i32 = tmp.trim().parse().unwrap();
        want.push(tmp);
    }

    want.reverse();
    let mut stack: Vec<i32> = Vec::new();

    let mut now = 0;
    let mut output = String::new();
    let mut flag = false;

    loop {
        if want.is_empty() {
            break;
        }
        if stack.is_empty() {
            let next = want.last().unwrap();
            for i in now+1..next+1 {
                stack.push(i);
                output.push('+');
                output.push('\n');
            }
            now = *next;
        } else if want.last() == stack.last() {
            stack.pop();
            want.pop();
            output.push('-');
            output.push('\n');


        } else if want.last() > stack.last() {
            let next = want.last().unwrap();
            for i in now+1..next+1 {
                stack.push(i);
                output.push('+');
                output.push('\n');
            }
            now = *next;
        } else {
            flag = true;
            break;
        }
    }

    if flag {
        println!("{}", "NO");
    } else {
        let mut stdout = io::BufWriter::new(io::stdout());
        write!(stdout, "{}", output).unwrap();
    }
}