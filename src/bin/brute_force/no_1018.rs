mod no_7568;

use std::io::{self, Write};
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut board: Vec<Vec<char>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let row: Vec<char> = line.trim().chars().collect();
        board.push(row);
    }

    let mut min_: i32 = 1_000_000_000;

    for i in 0..n-7 {
        for j in 0..m-7 {
            let mut cnt_w: i32 = 0;
            let mut cnt_b: i32 = 0;

            for k in i..i+8 {
                for l in j..j+8 {
                    let expect_w = if (k+l)%2 == 0 {'W'} else {'B'};
                    let expect_b = if (k+l)%2 == 0 {'B'} else {'W'};

                    if board[k][l] != expect_w {
                        cnt_w += 1;
                    }
                    if board[k][l] != expect_b {
                        cnt_b += 1;
                    }
                }
            }

            min_ = cmp::min(min_, cmp::min(cnt_w, cnt_b));
        }
    }
    println!("{}", min_);
}
