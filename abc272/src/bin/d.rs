use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
    };

    let mut vec: Vec<Vec<isize>> = vec![vec![-1; n]; n];
    let mut deque: VecDeque<Vec<(isize, isize)>> = VecDeque::new();
    let mut pos_list = Vec::new();

    for i in 1..=n {
        for j in 1..=n {
            if solve(1, 1, i as isize, j as isize) == m {
                pos_list.push((j as isize - 1, i as isize - 1));
            }
        }
    }

    vec[0][0] = 0;
    deque.push_back(vec![(0, 0)]);

    let mut count = 1;

    while !deque.is_empty() {
        let t_vec = deque.pop_front().unwrap();
        let mut n_vec = vec![];

        for (tx, ty) in t_vec {
            let mut pos = vec![];

            for (x, y) in pos_list.iter() {
                pos.push((tx - x, ty - y));
                pos.push((tx - x, ty + y));
                pos.push((tx + x, ty - y));
                pos.push((tx + x, ty + y));
                pos.push((tx - y, ty - x));
                pos.push((tx - y, ty + x));
                pos.push((tx + y, ty - x));
                pos.push((tx + y, ty + x));
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
                vec[*nx as usize][*ny as usize] = count;
                n_vec.push((*nx, *ny));
            }
        }

        if n_vec.is_empty() {
            break;
        }

        deque.push_back(n_vec);
        count += 1;
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
