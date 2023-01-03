use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let mut c = 1;
    let mut visited = vec![false; n + 1];
    let mut p = vec![0; n + 2];
    visited[1] = true;
    p[1] = c;

    for i in 2..=n {
        let mut min = 10000.0;
        let mut min_id = -1;

        for j in 1..=n {
            if visited[j] {
                continue;
            }

            let d = manhattan_distance(xy[c - 1].0, xy[c - 1].1, xy[j - 1].0, xy[j - 1].1);

            if d < min {
                min = d;
                min_id = j as isize;
            }
        }

        visited[min_id as usize] = true;
        p[i] = min_id as usize;
        c = min_id as usize;
    }

    p[n + 1] = 1;

    for i in 1..=n + 1 {
        println!("{}", p[i]);
    }
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> f64 {
    ((x1 - x2).abs() as f64 + (y1 - y2).abs() as f64).sqrt()
}
