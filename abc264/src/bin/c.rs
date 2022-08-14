use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    };

    let is_same = |x: &Vec<Vec<usize>>, y: &Vec<Vec<usize>>| {
        for i in 0..x.len() {
            for j in 0..x[0].len() {
                if x[i][j] != y[i][j] {
                    return false;
                }
            }
        }
        true
    };

    for bit1 in 0..1 << h1 {
        for bit2 in 0..1 << w1 {
            let mut is: Vec<usize> = vec![];
            let mut js: Vec<usize> = vec![];

            for i in 0..h1 {
                if bit1 >> i & 1 == 1 {
                    is.push(i);
                }
            }

            for j in 0..w1 {
                if bit2 >> j & 1 == 1 {
                    js.push(j);
                }
            }

            if is.len() != h2 || js.len() != w2 {
                continue;
            }

            let mut c: Vec<Vec<usize>> = vec![vec![0; w2]; h2];

            for i in 0..h2 {
                for j in 0..w2 {
                    c[i][j] = a[is[i]][js[j]];
                }
            }

            if is_same(&c, &b) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
