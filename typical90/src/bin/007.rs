use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q],
    };

    a.sort();

    for v in b.iter() {
        let t = a.binary_search(v).unwrap_or_else(|i| i);

        let mut ans = 100000000000000;

        for i in ((t as isize - 1).max(0) as usize)..(t + 1).min(n) {
            ans = ans.min(a[i] - v).abs();
        }

        println!("{}", ans);
    }
}
