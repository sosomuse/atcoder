use proconio::input;

fn main() {
    input! {
        abc: [usize; 3],
    };

    let mut ans = 0;
    for v in abc {
        ans += 7 - v;
    }

    println!("{}", ans);
}
