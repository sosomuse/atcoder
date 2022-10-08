use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, char); n],
    };

    let mut ans = 0;

    for i in 0..n {
        let (a, b) = ab[i];

        if b == 'E' {
            ans = ans.max(l - a);
        } else {
            ans = ans.max(a);
        }
    }

    println!("{}", ans);
}
