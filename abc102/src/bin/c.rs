use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    };

    for i in 0..n {
        a[i] -= (i + 1) as isize;
    }

    a.sort();

    let b = {
        if n % 2 == 1 {
            a[n / 2]
        } else {
            (a[n / 2 - 1] + a[n / 2]) / 2
        }
    };

    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - b).abs();
    }

    println!("{}", ans);
}
