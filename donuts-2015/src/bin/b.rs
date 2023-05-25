use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };

    let mut bci = vec![];

    for _ in 0..m {
        input! {
            b: usize,
            c: usize,
            i: [Usize1; c],
        };

        bci.push((b, c, i));
    }

    let perm = (0..n).combinations(9).collect_vec();

    let mut ans = 0;

    for p in perm {
        let mut tmp = 0;

        for i in 0..p.len() {
            tmp += a[p[i]];
        }

        for (b, c, i) in &bci {
            let mut cnt = 0;

            for j in 0..*c {
                if p.contains(&i[j]) {
                    cnt += 1;
                }
            }

            if cnt >= 3 {
                tmp += b;
            }
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
