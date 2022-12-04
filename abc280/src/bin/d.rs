use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut a = solve(k);
    a.sort();

    let mut ans = 10000000000000;

    for i in 0..a.len() {
        let v = a[i];

        if v == k {
            ans = std::cmp::min(ans, v);
            continue;
        }

        let mut ok = false;
        let mut tmp = k;

        for j in (2..v).rev() {
            if j >= tmp {
                ok = true;
                break;
            }
            tmp /= j;
        }

        if !ok {
            continue;
        }

        let b = k / v;
        let b2 = solve(b);
        let a2 = b2.iter().filter(|&&x| x < v).collect::<Vec<_>>();

        let mut seki = 1;

        for v in a2 {
            seki *= v;
        }

        if seki % b == 0 {
            println!("{}", v);
            return;
        }
    }

    println!("{}", ans);
}

fn solve(n: usize) -> Vec<usize> {
    let mut lst: Vec<usize> = vec![];

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            lst.push(i);
            if i != n / i {
                lst.push(n / i);
            }
        }

        i += 1;
    }

    lst
}
