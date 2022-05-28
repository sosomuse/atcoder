use proconio::input;

fn main() {
    input! {
        n: u32,
        mut d: [u32; n],
    }

    d.sort_by(|a, b| b.cmp(a));

    let mut ans = 1;
    let mut current = d[0];

    // dbg!(&d);

    for v in d {
        if current > v {
            ans += 1;
            current = v;
        }
    }

    println!("{}", ans);
}
