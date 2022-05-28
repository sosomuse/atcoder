use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    }

    let mut ans = 0;
    a.sort_by(|s, t| t.cmp(s));

    dbg!(&a);
    for i in 0..a.len() {
        let v = a[i];

        if i % 2 == 0 {
            ans += v;
        } else {
            ans -= v;
        }
    }

    print!("{}", ans);
}
