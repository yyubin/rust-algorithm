use std::io::{self, Write};
use std::collections::VecDeque;

fn bfs(n: usize, start: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>) {
    let mut q: VecDeque<i32> = VecDeque::new();

    q.push_back(start as i32);
    visited[start] = true;

    while q.len() > 0 {
        let now = q.pop_front().unwrap();

        for next in &graph[now as usize] {
            if 0 <= *next && *next < n as i32 && !visited[*next as usize]{
                visited[*next as usize] = true;
                q.push_back(*next);
            }
        }
    }

}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
    let mut visited: Vec<bool> = vec![false; n];

    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let a = iter.next().unwrap() - 1;
        let b = iter.next().unwrap() - 1;
        graph[a].push(b as i32);
        graph[b].push(a as i32);
    }

    let mut res = 0;
    for i in 0..n {
        if !visited[i] && !graph[i].is_empty() {
            bfs(n, i, &graph, &mut visited);
            res += 1;
        }
    }

    for i in 0..n {
        if !visited[i] && graph[i].is_empty() {
            res += 1;
        }
    }

    println!("{}", res);
}