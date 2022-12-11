use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut ps: Vec<(usize, usize)> = vec![];

    {
        let mut x = k;
        let mut i = 2;
        while i * i <= x {
            if x % i != 0 {
                i += 1;
                continue;
            };
            let mut cnt = 0;
            while x % i == 0 {
                x /= i;
                cnt += 1;
            }
            ps.push((i, cnt));

            i += 1;
        }

        if x != 1 {
            ps.push((x, 1))
        };
    }

    let mut ac = k;
    let mut wa = 0;

    while ac - wa > 1 {
        let wj = (ac + wa) / 2;
        let mut ok = true;

        for (p, cnt) in ps.iter() {
            if f(wj, *p) < *cnt {
                ok = false;
            }
        }

        if ok {
            ac = wj;
        } else {
            wa = wj;
        }
    }

    println!("{}", ac);
}

fn f(mut n: usize, p: usize) -> usize {
    if n == 0 {
        return 0;
    };
    n /= p;
    return n + f(n, p);
}
