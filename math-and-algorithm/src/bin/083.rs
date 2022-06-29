use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    };

    a.sort();
    b.sort();

    let mut ans = 0;

    for i in 0..n {
        let v = a[i];
        let pos = b.binary_search(&v).unwrap_or_else(|i| i.max(1) - 1);
        let target = b[pos];
        ans += (v as isize - target as isize).abs();
        b.remove(pos);
    }

    println!("{}", ans);
}
