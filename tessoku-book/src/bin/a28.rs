use proconio::input;

fn main() {
    input! {
        n: usize,
        ta: [(char, isize); n],
    };

    let mut ans: isize = 0;

    for i in 0..n {
        let (t, a) = ta[i];

        match t {
            '+' => ans += a,
            '-' => ans -= a,
            '*' => ans *= a,
            _ => panic!(),
        }

        if ans < 0 {
            ans += 10000;
        }

        ans %= 10000;

        println!("{}", ans);
    }
}
