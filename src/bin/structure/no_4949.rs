use std::io::{self, Write};

fn main() {
    let mut output = String::new();
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let s = s.trim_end();
        if s == "." {
            break
        }

        let mut stack: Vec<char> = Vec::new();
        let mut flag = false;

        for c in s.chars() {
            if c == '(' || c == '[' {
                stack.push(c);
                continue;
            }

            if c == ')' || c == ']' {
                if stack.len() == 0 {
                    flag = true;
                    break;
                }

                if c == ')' {
                    let last = stack.last().unwrap();
                    if *last == '(' {
                        stack.pop();
                    } else {
                        flag = true;
                        break;
                    }
                } else if c == ']' {
                    let last = stack.last().unwrap();
                    if *last == '[' {
                        stack.pop();
                    } else {
                        flag = true;
                        break;
                    }
                }
            }

        }

        let res = if !flag && stack.len() == 0 {"yes"} else {"no"};
        output.push_str(&format!("{}\n", res));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}