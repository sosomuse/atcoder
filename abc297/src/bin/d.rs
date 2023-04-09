use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    };

    let mut ans = 0;

    while a != b {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }

        let diff = a - b;
        let count = diff / b;

        ans += count;
        a -= count * b;

        if a > b {
            a -= b;
            ans += 1;
        }
    }

    println!("{}", ans);
}
