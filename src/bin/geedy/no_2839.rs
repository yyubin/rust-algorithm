use std::io::{self, Write};

fn main() {
    let mut stdout = io::BufWriter::new(io::stdout());
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut n: i32 = n.trim().parse().unwrap();

    let mut cnt = 0;
    let mut flag = false;

    while n >= 0 {
        if n%5 == 0 {
            cnt += n/5;
            flag = true;
            break;
        }

        cnt += 1;
        n -= 3;
    }
    
    let output = if flag {cnt} else {-1};
    write!(stdout, "{}", output).unwrap();
}