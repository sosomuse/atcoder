use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u8; w]; h],
    };

    let is_valid = |r: usize, c: usize, a: &Vec<Vec<u8>>| {
        let v = a[r][c];
        let mut ok = false;

        dbg!(r, c, a);

        if r > 0 && a[r - 1][c] == v {
            ok = true;
        }

        if r + 1 < h && a[r + 1][c] == v {
            ok = true;
        }

        if c > 0 && a[r][c - 1] == v {
            ok = true;
        }

        if c + 1 < w && a[r][c + 1] == v {
            ok = true;
        }

        return ok;
    };

    let mut last: Option<usize> = None;
    let mut count = 0;

    for i in 0..h {
        for j in 0..w {
            let valid = { !is_valid(i, j, &a) };
            if !valid {
                if let Some(l) = last {
                    if i - l == 1 {
                        println!("-1");
                        return;
                    }
                }

                for k in 0..w {
                    if a[i][k] == 1 {
                        a[i][k] = 0;
                    } else {
                        a[i][k] = 1;
                    }
                }

                last = Some(i);
                count += 1;
            }
        }
    }

    println!("{}", count);
}
