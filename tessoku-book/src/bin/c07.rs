use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
    };

    c.sort();
    let mut vec = vec![0; n + 1];

    for i in 0..n {
        vec[i + 1] = vec[i] + c[i];
    }

    for _ in 0..q {
        input! {
            x: usize,
        };

        let ans = vec.binary_search(&x).unwrap_or_else(|i| i - 1);

        println!("{}", ans);
    }
}
