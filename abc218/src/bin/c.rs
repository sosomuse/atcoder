use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    };

    let mut s_p = vec![];
    let mut t_p = vec![];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                s_p.push((i as isize, j as isize));
            }
            if t[i][j] == '#' {
                t_p.push((i as isize, j as isize));
            }
        }
    }

    if solve(&s_p, &t_p) {
        println!("Yes");
        return;
    }

    let mut s_p = vec![];

    for j in (0..n).rev() {
        for i in 0..n {
            if s[i][j] == '#' {
                s_p.push(((n - 1 - j) as isize, i as isize));
            }
        }
    }

    if solve(&s_p, &t_p) {
        println!("Yes");
        return;
    }

    let mut s_p = vec![];

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if s[i][j] == '#' {
                s_p.push(((n - 1 - i) as isize, (n - 1 - j) as isize));
            }
        }
    }

    if solve(&s_p, &t_p) {
        println!("Yes");
        return;
    }

    let mut s_p = vec![];

    for j in 0..n {
        for i in (0..n).rev() {
            if s[i][j] == '#' {
                s_p.push((j as isize, (n - 1 - i) as isize));
            }
        }
    }

    if solve(&s_p, &t_p) {
        println!("Yes");
        return;
    }

    println!("No");
}

fn solve(s_p: &Vec<(isize, isize)>, t_p: &Vec<(isize, isize)>) -> bool {
    if s_p.len() != t_p.len() {
        return false;
    }

    let (s_x, s_y) = s_p[0];
    let (t_x, t_y) = t_p[0];

    for i in 1..s_p.len() {
        let (s_x2, s_y2) = s_p[i];
        let (t_x2, t_y2) = t_p[i];

        if (s_x2 - s_x) != (t_x2 - t_x) || (s_y2 - s_y) != (t_y2 - t_y) {
            return false;
        }
    }

    true
}
