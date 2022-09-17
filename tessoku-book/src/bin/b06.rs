use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };

    let mut s = vec![0; n + 1];

    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        let x = (s[r] - s[l - 1]) * 2;
        let y = r - (l - 1);

        if x == y {
            println!("draw");
        } else if x > y {
            println!("win");
        } else {
            println!("lose");
        }
    }
}
