use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };

    let mut ans: usize = 10000000000000000000;
    let mut s: usize = 0;
    let mut l: usize = 10000000000000000000;

    for i in 0..n {
        s += ab[i].0 + ab[i].1;
        l = l.min(ab[i].1);
        if x < i + 1 {
            break;
        }

        let now = s + l * (x - i - 1);
        ans = ans.min(now);
    }

    println!("{}", ans);
}
