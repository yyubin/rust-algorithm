use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();

    let mut input = String::new();
    let mut output = String::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let now = input.trim();
        let mut st: Vec<char> = Vec::new();
        let mut flag = false;

        for c in now.chars() {
            if c == '[' || c == '(' {
                st.push(c);
                continue;
            }

            if st.len() == 0 {
                flag = true;
                break;
            }

            let last = st.last().unwrap();
            if c == ']' && *last == '[' {
               st.pop();
            } else if c == ')' && *last == '(' {
                st.pop();
            } else {
                flag = true;
                break;
            }
        }
        let res = if !flag && st.is_empty() {"YES"} else {"NO"};
        output.push_str(&format!("{}\n", res));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{output}").unwrap();
}