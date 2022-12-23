use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    };

    let mut ans = a;

    for _ in 1..k {
        if ans == n {
            ans = 1;
        } else {
            ans += 1
        }
    }

    println!("{}", ans)
}
