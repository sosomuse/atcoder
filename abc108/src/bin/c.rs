use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let mut m = 0;

    for i in 1..=n {
        if i % k == 0 {
            m += 1;
        }
    }

    let mut ans = m * m * m;

    if k % 2 == 0 {
        let mut c = 0;
        for i in 1..=n {
            if i % k == k / 2 {
                c += 1;
            }
        }
        ans += c * c * c;
    }

    println!("{}", ans);
}
