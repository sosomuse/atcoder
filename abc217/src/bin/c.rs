use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };

    let mut q = vec![0; n + 1];

    for i in 1..=n {
        let v = p[i - 1];
        q[v] = i;
    }

    for i in 1..=n {
        print!("{} ", q[i]);
    }
}
