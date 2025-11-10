mod no_2178;

use std::io::{self, Write};
use std::collections::{HashMap, VecDeque};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    let d = (iter.next().unwrap(), iter.next().unwrap());

    let mut m: HashMap<i32, i32> = HashMap::new();

    for _ in 0..d.0 + d.1 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        m.insert(iter.next().unwrap(), iter.next().unwrap());
    }

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; 101];
    q.push_back((1, 0));
    visited[1] = true;

    while q.len() > 0 {
        let (now, cnt) = q.pop_front().unwrap();
        if now == 100 {
            println!("{}", cnt);
            break;
        }

        for i in 0..7 {
            let next = now + i;
            if next <= 100 && !visited[next as usize]{
                visited[next as usize] = true;
                if m.contains_key(&next) {
                    q.push_back((*m.get(&next).unwrap(), cnt+1));
                } else {
                    q.push_back((next, cnt+1));
                }
            }
        }

    }
}