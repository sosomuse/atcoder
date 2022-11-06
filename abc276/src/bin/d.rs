use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut gc = gcd(a[0], a[1]);

    for i in 2..n {
        gc = gcd(gc, a[i]);
    }

    let mut ans = 0;

    for i in 0..n {
        let v = a[i] / gc;
        if let Some(x) = get_count(v) {
            ans += x;
        } else {
            println!("-1");
            return;
        }
    }

    // dbg!(ans, gc);

    println!("{}", ans);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    let mut r = a % b;
    while r != 0 {
        a = b;
        b = r;
        r = a % b;
    }

    return b;
}

fn get_count(mut n: usize) -> Option<usize> {
    let mut count = 0;

    while n != 1 {
        if n % 3 == 0 {
            n /= 3;
            count += 1;
        } else if n % 2 == 0 {
            n /= 2;
            count += 1;
        } else {
            return None;
        }
    }

    Some(count)
}
