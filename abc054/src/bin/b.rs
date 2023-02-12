use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
        b: [Chars; m],
    };

    for i in 0..n - m + 1 {
        for j in 0..n - m + 1 {
            let mut ok = true;

            for k in 0..m {
                for l in 0..m {
                    if a[i + k][j + l] != b[k][l] {
                        ok = false;
                        break;
                    }
                }

                if !ok {
                    break;
                }
            }

            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
