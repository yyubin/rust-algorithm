use std::io::{self, Write};

fn main() {
    let mut dp: Vec<i32> = vec![0; 12];
    dp[0] = 1;
    dp[1] = 1;
    dp[2] = 2;

    for i in 3..12 {
        dp[i] += dp[i-1] + dp[i-2] + dp[i-3];
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    let mut stdout = io::BufWriter::new(io::stdout());

    for i in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        let _ = write!(stdout, "{}\n", dp[n as usize]);
    }
}