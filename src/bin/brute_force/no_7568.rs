use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut v: Vec<(i32, i32)> = Vec::new();

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let t = (iter.next().unwrap(), iter.next().unwrap());
        v.push((t.0, t.1));
    }

    let mut res: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        let mut top = 0;
        for j in 0 ..n {
            if i == j {
                continue;
            }
            if v[i as usize].0 < v[j as usize].0 && v[i as usize].1 < v[j as usize].1 {
                top += 1;
            }
        }
        res.push((i, 1+top));
    }

    let mut output = String::new();
    for i in res {
        output.push_str(&format!("{} ", i.1));
    }

    let mut stdout = io::BufWriter::new(io::stdout());
    write!(stdout, "{}", output).unwrap();
}