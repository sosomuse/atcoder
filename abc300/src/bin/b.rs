use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
        b: [Chars; h],
    }

    for s in 0..h {
        for t in 0..w {
            let mut is_match = true;
            for i in 0..h {
                for j in 0..w {
                    if a[i][j] != b[(i + s) % h][(j + t) % w] {
                        is_match = false;
                        break;
                    }
                }
                if !is_match {
                    break;
                }
            }
            if is_match {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
