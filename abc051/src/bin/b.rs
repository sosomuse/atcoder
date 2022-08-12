use proconio::input;

fn main() {
    input! {
        k: usize,
        s: usize,
    };

    let mut ans = 0;

    for i in 0..=k {
        for j in 0..=k {
            if i + j > s {
                break;
            }

            let l = s - i - j;

            if l <= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
