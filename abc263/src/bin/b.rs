use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    };

    let last = p.last().unwrap();
    let mut t = *last;

    for i in 0..p.len() {
        if t == 1 {
            println!("{}", i + 1);
            return;
        }

        t = p[t - 2];
    }
}
