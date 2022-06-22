use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        m: usize,
        b: [usize; m],
    }

    let mut vec = vec![0; n + 1];

    for i in 0..n - 1 {
        vec[i + 1] = vec[i] + a[i];
    }

    let mut ans = 0;

    for i in 0..m {
        if i == 0 {
            continue;
        }

        let x = b[i - 1].max(b[i]);
        let y = b[i].min(b[i - 1]);

        ans += vec[x - 1] - vec[y - 1];
    }

    println!("{}", ans);
}
