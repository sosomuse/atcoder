use proconio::input;

fn main() {
    input! {
        mut a: u64,
        b: u64,
        k: u64,
    }

    let mut ans = 0;

    while a < b {
        a *= k;
        ans += 1;
    }

    println!("{}", ans)
}
