use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut c = vec![];

    for _ in 0..m {
        input! {
            k: usize,
            a: [usize; k],
        };

        c.push(a);
    }

    let mut ans = 0;

    for bit in 0..1 << m {
        let mut set = std::collections::HashSet::new();

        for i in 0..m {
            if bit >> i & 1 == 1 {
                for j in 0..c[i].len() {
                    set.insert(c[i][j]);
                }
            }
        }

        if set.len() == n {
            ans += 1;
        }
    }

    println!("{}", ans)
}
