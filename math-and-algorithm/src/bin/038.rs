use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut vec = vec![0; n + 1];

    for i in 0..n {
        vec[i + 1] = vec[i] + a[i];
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        println!("{}", vec[r] - vec[l - 1]);
    }
}
