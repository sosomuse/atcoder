use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };

    let get_score = |a: usize, b: usize| {
        let mut cnt = 0;

        for i in 0..n {
            let (_a, _b) = ab[i];
            if a <= _a && _a <= a + k && b <= _b && _b <= b + k {
                cnt += 1;
            }
        }

        cnt
    };

    let mut ans = 0;

    for i in 1..=100 {
        for j in 1..=100 {
            ans = std::cmp::max(ans, get_score(i, j));
        }
    }

    println!("{}", ans);
}
