use proconio::input;

fn main() {
    input! {
        mut n: isize,
        k: usize,
        mut a: [usize; k],
    };

    a.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut count = 0;

    while n > 0 {
        a.retain(|&v| v <= n as usize);

        if a.len() == 0 {
            break;
        }

        if count % 2 == 0 {
            ans += a[0];
        }

        n -= a[0] as isize;
        count += 1;
    }

    println!("{}", ans);
}
