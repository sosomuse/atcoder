use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        k: usize,
        mut t: [usize; n],
    };

    t.sort();

    let mut ans: usize = 0;
    let mut p = 0;
    let mut count = 0;

    for i in 0..n {
        let v = t[i];

        if v > p || count >= c {
            ans += 1;
            p = v + k;
            count = 1;

            continue;
        }

        count += 1;
    }

    println!("{}", ans);
}
