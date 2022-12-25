use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };

    for _ in 0..q {
        input! {
            t: usize,
            k: usize,
        };

        if t == 1 {
            input! {
                x: usize,
            };

            a[k - 1] = x;
        } else {
            println!("{}", a[k - 1])
        }
    }
}
