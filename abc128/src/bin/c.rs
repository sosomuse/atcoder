use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut ks = vec![];

    for _ in 0..m {
        input! {
            k: usize,
            s: [usize; k],
        };
        ks.push(s);
    }

    input! {
        p: [usize; m],
    };

    let mut ans = 0;

    for bit in 0..1 << n {
        let mut ok = true;

        for j in 0..m {
            let mut count = 0;

            for k in 0..ks[j].len() {
                if bit & (1 << (ks[j][k] - 1)) != 0 {
                    count += 1;
                }
            }

            if count % 2 != p[j] {
                ok = false;
            }
        }

        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
