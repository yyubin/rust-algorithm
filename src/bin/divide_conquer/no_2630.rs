use std::io::{self, Write};

static mut CNT0: i32 = 0;
static mut CNT1: i32 = 0;
static mut BOARD: Vec<Vec<i32>> = Vec::new();

fn recursion(x: i32, y: i32, now: i32) {
    unsafe {
        let target = BOARD[x as usize][y as usize];
        let mut cut = false;

        for i in x..x + now {
            if cut { break; }
            for j in y..y + now {
                if BOARD[i as usize][j as usize] != target {
                    cut = true;
                    break;
                }
            }
        }
        if cut {
            let half = now / 2;
            recursion(x, y, half);
            recursion(x + half, y, half);
            recursion(x, y + half, half);
            recursion(x + half, y + half, half);
            return;
        }

        if target == 1 {
            CNT1 += 1;
        } else {
            CNT0 += 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut board = vec![vec![0; n]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        board[i] = row;
    }

    unsafe {
        BOARD = board;
        recursion(0, 0, n as i32);
        let cnt0: i32 = CNT0;
        let cnt1: i32 = CNT1;
        println!("{}\n{}", cnt0, cnt1);
    }
}