use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    };

    let sum = a.iter().sum::<usize>();
    let mut ans = x / sum * n;
    let mut res = x % sum;

    for i in 0..n {
        ans += 1;

        if res < a[i] {
            println!("{}", ans);
            return;
        }

        res -= a[i];
    }
}
