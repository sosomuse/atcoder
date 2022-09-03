use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
    };

    let mut vec: Vec<isize> = vec![0; n + 1];
    let mut ans = 0;

    for i in 1..n {
        vec[i] = a[i] - a[i - 1];
        ans += vec[i].abs();
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
            v: isize,
        };

        let prev = vec[l - 1].abs() + vec[r].abs();
        if l >= 2 {
            vec[l - 1] += v;
        }
        if r <= n - 1 {
            vec[r] -= v;
        }
        let next = vec[l - 1].abs() + vec[r].abs();
        ans += next - prev;
        println!("{}", ans);
    }
}
