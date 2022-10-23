use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let mut counts = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                counts[j] += 1;
            }
        }
    }

    for i in 0..w {
        print!("{} ", counts[i]);
    }
}
