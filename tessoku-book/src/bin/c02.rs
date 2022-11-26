use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort_by(|a, b| b.cmp(a));

    println!("{}", a[0] + a[1]);
}
