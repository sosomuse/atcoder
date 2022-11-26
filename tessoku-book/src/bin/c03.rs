use proconio::input;

fn main() {
    input! {
        d: usize,
        x: isize,
        a: [isize; d - 1],
        q: usize,
    };

    let mut m: Vec<isize> = vec![0; d];
    m[0] = x;

    for i in 0..a.len() {
        m[i + 1] = m[i] + a[i];
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        }

        if m[s - 1] == m[t - 1] {
            println!("Same");
        } else if m[s - 1] > m[t - 1] {
            println!("{}", s);
        } else {
            println!("{}", t);
        }
    }
}
