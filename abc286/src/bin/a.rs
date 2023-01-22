use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        _: usize,
        mut a: [usize; n],
    };

    for i in p..=q {
        a.swap(i - 1, r + i - p - 1);
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
}
