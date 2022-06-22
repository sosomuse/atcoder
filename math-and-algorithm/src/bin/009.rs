use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut vec = vec![false; s + 1];
    vec[0] = true;

    for _ in 0..n {
        input! {
            a: usize,
        }
        if a <= s {
            for j in (0..(s - a + 1)).rev() {
                if vec[j] {
                    vec[j + a] = true
                }
            }
        }
    }

    if vec[s] {
        println!("Yes");
    } else {
        println!("No");
    }
}
