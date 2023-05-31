use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    let mut k = 0;

    for i in 1..=n {
        if i * i <= n {
            k = i;
        } else {
            break;
        }
    }

    for i in 1..=k {
        ans += ((n / i) - (n / (i + 1))) * i;
    }
    for i in 1..=n / (k + 1) {
        ans += n / i;
    }

    println!("{}", ans);
}
