use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    };

    a.sort_by(|a, b| b.cmp(a));
    let mut dp = vec![false; x + 1];
    let mut set = std::collections::HashSet::new();
    dp[0] = true;

    for i in 0..m {
        set.insert(b[i]);
    }

    for i in 0..x {
        for j in 0..n {
            if i + a[j] <= x && dp[i] && !set.contains(&(i + a[j])) {
                dp[i + a[j]] = true;
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
