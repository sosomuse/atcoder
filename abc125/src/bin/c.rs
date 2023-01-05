use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    a.sort();

    let mut g = gcd(a[0], a[1]);
    let mut t = None;

    for i in 2..n {
        let prev = g;
        g = gcd(g, a[i]);

        if g < prev {
            t = Some(i);
        }
    }

    if let Some(i) = t {
        let mut max = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            max = a[j].max(max);
        }

        let mut b = a.clone();
        b[i] = max;

        println!("{}", vec_gcd(&b));
        return;
    }

    let mut c = a.clone();
    let mut d = a.clone();
    let max = a[n - 1];

    c[0] = max;
    d[1] = max;

    let ans = vec_gcd(&c).max(vec_gcd(&d));
    println!("{}", ans);
}

fn vec_gcd(a: &Vec<usize>) -> usize {
    let mut g = a[0];
    for i in 1..a.len() {
        g = gcd(g, a[i]);
    }
    g
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
