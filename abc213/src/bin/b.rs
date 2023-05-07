use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut s = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (i + 1, x))
        .collect::<Vec<_>>();

    s.sort_by_key(|&(_, x)| x);

    println!("{}", s[n - 2].0)
}
