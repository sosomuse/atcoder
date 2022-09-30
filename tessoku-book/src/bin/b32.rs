use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    };

    let mut dp = vec![false; n + 1];

    for i in 0..n {
        if dp[i] {
            continue;
        }
        for &a_i in a.iter() {
            if i + a_i <= n {
                dp[i + a_i] = true;
            }
        }
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
