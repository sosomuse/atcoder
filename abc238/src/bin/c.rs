use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let c = n.to_string().len();
    let mut ans = 0;

    let _mod = 998244353;

    for i in 1..=c {
        if i != c {
            let x = (10_usize.pow(i as u32) - 10_usize.pow(i as u32 - 1)) % _mod;
            ans += (1 + x) * x / 2;
            ans %= _mod;
        } else {
            let x = (n - (10_usize.pow(i as u32 - 1) - 1)) % _mod;
            ans += (1 + x) * x / 2;
            ans %= _mod;
        }
    }

    println!("{}", ans);
}
