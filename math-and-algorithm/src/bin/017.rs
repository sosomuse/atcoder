use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut ans = lcm(a[0], a[1]);

    for i in 1..a.len() {
        let v = a[i];
        ans = lcm(ans, v);
    }

    println!("{}", ans);
}

fn lcm(mut a1: usize, mut b1: usize) -> usize {
    let a2 = a1;
    let b2 = b1;
    while a1 >= 1 && b1 >= 1 {
        if a1 > b1 {
            a1 %= b1;
        } else {
            b1 %= a1;
        }
    }

    return a2 / (a1 + b1) * b2;
}
