use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = 0;
    ans += n;
    let mut b = 2;
    let mut c = n / 2;

    while b <= c {
        ans += c - b + 1;
        b += 1;
        c = n / b;
    }

    let mut a = 2;
    let mut b = 2;
    let mut c = n / (a * b);

    while (a * b) <= c {
        ans += c - b + 1;
        a += 1;
        b += 1;
        c = n / (a * b);
    }

    println!("{}", ans);
}
