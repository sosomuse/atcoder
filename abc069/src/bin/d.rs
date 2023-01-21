use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [usize; n],
    };

    let mut s = vec![vec![0; w]; h];
    let mut aic: (usize, usize) = (0, 0);

    for i in 0..h {
        for j in 0..w {
            let k = {
                if i % 2 == 0 {
                    j
                } else {
                    w - j - 1
                }
            };

            let (ai, ac) = aic;
            let t = a[ai];

            s[i][k] = ai + 1;

            if ac + 1 == t {
                aic = (ai + 1, 0);
            } else {
                aic = (ai, ac + 1);
            }
        }
    }

    for vec in s {
        for v in vec {
            print!("{} ", v);
        }

        println!();
    }
}
