use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let m: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let s: &str = input.trim();

    let mut cnt = 0;
    let mut res = 0;
    let mut i = 1;
    let bytes = s.as_bytes();

    while i < m - 1 {
        if bytes[i-1] == b'I' && bytes[i] == b'O' && bytes[i+1] == b'I' {
            cnt += 1;
            if cnt >= n {
                res += 1;
            }
            i += 2;
        } else {
            cnt = 0;
            i += 1;
        }
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", res).unwrap();
}