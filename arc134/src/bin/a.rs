use proconio::input;

fn main() {
    input! {
        (n, l, w): (i64, i64, i64),
        a: [i64; n],
    }

    let mut current: i64 = 0;
    let mut ans: i64 = 0;

    for i in a {
        if current < i {
            ans += (i - current + w - 1) / w;
        }
        current = i + w;
    }

    if l - current > 0 {
        ans += (l - current + w - 1) / w;
    }

    println!("{}", ans);
}
