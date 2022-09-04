use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    };

    let mut ans: isize = 0;
    let mut next: isize = 0;
    let mut sum: isize = 0;

    sum = a[0..m].iter().sum();
    next = a[0..m]
        .iter()
        .enumerate()
        .map(|(i, x)| x * (i + 1) as isize)
        .sum();
    ans = next;

    for i in 1..=n - m {
        let l = i + m - 1;
        let x = a[l];

        next = next - sum + x * m as isize;
        sum = sum - a[i - 1] + x;
        ans = ans.max(next);
    }

    println!("{}", ans);
}
