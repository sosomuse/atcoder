use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
    };

    let mut vec: Vec<Vec<isize>> = vec![vec![-1; n]; n];
    let mut deque: VecDeque<(isize, isize)> = VecDeque::new();
    let mut pos_list = Vec::new();

    for i in 1..=n {
        for j in 1..=n {
            if solve(1, 1, i as isize, j as isize) == m {
                pos_list.push((j as isize - 1, i as isize - 1));
            }
        }
    }

    vec[0][0] = 0;
    deque.push_back((0, 0));

    while !deque.is_empty() {
        let (tx, ty) = deque.pop_front().unwrap();

        let mut pos = vec![];

        for (x, y) in pos_list.iter() {
            pos.push((tx - x, ty - y));
            pos.push((tx - x, ty + y));
            pos.push((tx + x, ty - y));
            pos.push((tx + x, ty + y));
        }

        for (nx, ny) in pos.iter() {
            if *nx < 0 || n as isize <= *nx {
                continue;
            }
            if *ny < 0 || n as isize <= *ny {
                continue;
            }

            if vec[*nx as usize][*ny as usize] != -1 {
                continue;
            }
            vec[*nx as usize][*ny as usize] = vec[tx as usize][ty as usize] + 1;
            deque.push_back((*nx, *ny));
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", vec[i][j]);
        }

        println!();
    }
}

fn solve(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}
