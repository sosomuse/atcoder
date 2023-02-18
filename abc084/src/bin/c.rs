use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(usize, usize, usize); n-1],
    }

    for i in 0..n - 1 {
        let mut t = 0;

        for j in i..n - 1 {
            let (c, s, f) = a[j];
            if t < s {
                t = s;
            } else if t % f == 0 {
                // do nothing
            } else {
                t += f - (t % f);
            }
            t += c;
        }

        println!("{}", t);
    }
    println!("0");
}
