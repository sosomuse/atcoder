use proconio::input;

fn main() {
    input! {
        n: u32,
        mut d: [u32; n],
    }

    d.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut current = 0;

    for v in d {
        if v > current {
            ans += 1;
            current = v;
        }
    }

    println!("{}", ans);
}
