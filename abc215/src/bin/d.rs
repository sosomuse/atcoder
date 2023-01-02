use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };

    a.sort();

    // 2以上M以下の整数に対して、Aの約数 or Aの倍数が存在しないものの個数を求める
    let mut s = vec![true; 100001];

    for i in 0..n {
        let v = a[i];

        let lst = prime_factorize(v);

        for mut t in lst {
            if !s[t] {
                continue;
            }
            let c = t;

            while t <= m {
                s[t] = false;
                t += c;
            }
        }
    }

    let mut ans = vec![];

    for i in 1..=m {
        if s[i] {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    for i in 0..ans.len() {
        print!("{} ", ans[i]);
    }
}

fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];

    for p in 2..=n {
        if p * p > n {
            break;
        }

        if n % p != 0 {
            continue;
        }

        while n % p == 0 {
            n /= p;
        }

        ans.push(p);
    }

    if n != 1 {
        ans.push(n);
    }

    ans
}
