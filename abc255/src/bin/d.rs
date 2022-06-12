use proconio::input;

fn main() {
    input! {
        n: u64,
        q: u64,
        mut a: [u64; n],
    }

    a.sort();

    for _ in 0..q {
        input! {
            x: u64,
        }

        let b = a.binary_search(&x).unwrap_or_else(|i| i);
        dbg!(b);
        let mins = a[..b].iter().sum::<u64>() / a[..b].len() as u64;
        let maxs = a[b..].iter().sum::<u64>() / a[..b].len() as u64;

        let ans = mins + maxs;

        println!("{}", ans);
    }
}
