use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }

    let mut a_x: isize = 0;
    let mut a_y: isize = 0;
    let mut b_x: isize = 0;
    let mut b_y: isize = 0;

    for i in 0..w {
        for j in 0..h {
            if s[j][i] == 'o' {
                if a_x == 0 {
                    a_y = j as isize + 1;
                    a_x = i as isize + 1;
                } else {
                    b_y = j as isize + 1;
                    b_x = i as isize + 1;
                }
            }
        }
    }

    println!("{}", (b_x - a_x).abs() + (b_y - a_y).abs());
}
