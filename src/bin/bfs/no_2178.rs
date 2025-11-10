use std::io;
use std::collections::{VecDeque};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

    let mut board: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        board.push(row);
    }

    let mut q: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let dx: Vec<i32> = vec![1, -1, 0, 0]; let dy: Vec<i32> = vec![0, 0, 1, -1];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
    q.push_back((0, 0, 1));
    visited[0][0] = true;

    while q.len() > 0 {
        let (x, y, cnt) = q.pop_front().unwrap();
        if x == (n as i32) - 1 && y == (m as i32) - 1 {
            println!("{}", cnt);
            break;
        }

        for i in 0..4 {
            let nx = x + dx[i];
            let ny = y + dy[i];

            if 0 <= nx && nx < n as i32 && 0 <= ny && ny < m as i32 && !visited[nx as usize][ny as usize] && board[nx as usize][ny as usize] == 1 {
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx, ny, cnt+1));
            }
        }
    }

}