use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }

    a.sort();

    for _ in 0..q {
        input! {
            x: usize,
        };

        let ans = a.binary_search(&x).unwrap_or_else(|e| e);

        println!("{}", a.len() - ans);
    }
}
