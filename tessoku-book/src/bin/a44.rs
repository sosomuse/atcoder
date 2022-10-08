use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    };

    let mut a = vec![0; n];

    for i in 0..n {
        a[i] = i + 1;
    }

    let mut is_rev = false;

    for _ in 0..q {
        input! {
            t: usize,
        };

        match t {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                };

                if is_rev {
                    a[n - x] = y;
                } else {
                    a[x - 1] = y;
                }
            }
            2 => is_rev = !is_rev,
            3 => {
                input! {
                    x: usize,
                };

                println!("{}", if is_rev { a[n - x] } else { a[x - 1] });
            }
            _ => unreachable!(),
        }
    }
}
