use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut ans = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == x {
                    ans += 1;
                }
            }
        }
    }

    print!("{}", ans);
}
