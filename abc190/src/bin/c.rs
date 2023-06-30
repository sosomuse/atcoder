use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    };

    let mut ans = 0;
    for bit in 0..1 << k {
        let mut exsits = vec![false; n];
        let mut cnt = 0;

        for i in 0..k {
            if bit >> i & 1 == 0 {
                exsits[cd[i].0] = true;
            } else {
                exsits[cd[i].1] = true;
            }
        }
        for i in 0..m {
            if exsits[ab[i].0] && exsits[ab[i].1] {
                cnt += 1;
            }
        }
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
