use proconio::input;

fn main() {
    input! {
        a: [u64; 64],
    }

    let mut ans: u128 = 0;

    for i in 0..64 {
        if a[i] == 1 {
            ans += 1u128 << i;
        }
    }

    println!("{}", ans);
}
