use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut graph: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());

        for j in 0..n {
            let x = iter.next().unwrap();
            if x == 1 {
                graph[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if graph[i][k] == 1 && graph[k][j] == 1 {
                    graph[i][j] = 1;
                }
            }
        }
    }

    let mut output = String::new();
    for i in graph {
        for j in i {
            output.push_str(&format!("{} ", j));
        }
        output.push('\n');
    }
    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}