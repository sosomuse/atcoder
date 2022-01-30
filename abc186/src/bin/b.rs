use proconio::input;

fn main() {
    input! {
        (h, w): (u32, u32),
        a: [[u32; h]; w],
    }

    let mut min: u32 = 100;
    let mut ans: u32 = 0;

    for v in &a {
        for t in v {
            min = min.min(*t);
        }
    }

    for v in &a {
        for t in v {
            ans += t - min;
        }
    }

    println!("{}", ans);
}
