use std::io::{self, Write};
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let (n, k) = (iter.next().unwrap(), iter.next().unwrap());

    let mut visited = [false; 100001];
    visited[n as usize] = true;

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((n, 0));

    while !q.is_empty() {
        let (now, cnt) = q.pop_front().unwrap();
        if now == k {
            println!("{}", cnt);
            break;
        }

        for i in [now+1, now-1, now*2] {
            if 0 <= i && i <= 100000 && !visited[i as usize] {
                visited[i as usize] = true;
                q.push_back((i, cnt+1));
            }
        }

    }
}