use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut counts = vec![vec![0usize; 10]; 10];

    for i in 1..=n {
        let mut x = i;
        let mut top = 0;
        while x > 0 {
            top = x % 10;
            x /= 10;
        }
        let btm = i % 10;
        counts[top][btm] += 1;
    }

    let mut ans = 0;

    for top in 0..10 {
        for btm in 0..10 {
            ans += counts[top][btm] * counts[btm][top];
        }
    }

    println!("{}", ans);
}
