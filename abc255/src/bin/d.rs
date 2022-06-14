use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    a.sort();

    let mut rw = vec![0; n + 1];

    for (i, a) in a.iter().enumerate() {
        rw[i + 1] = rw[i] + a;
    }

    for _ in 0..q {
        input! {
            x: usize,
        }

        let b = a.binary_search(&x).unwrap_or_else(|i| i);

        let mut ans = x * (b);
        ans += rw[n] - rw[b];
        ans -= rw[b];
        ans -= x * (n - b);
        println!("{}", ans);
    }
}
