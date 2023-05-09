use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut a_with_index = a.into_iter().enumerate().collect::<Vec<_>>();

    a_with_index.sort_by_key(|&(_, x)| x);

    println!("{}", a_with_index[n - 2].0 + 1);
}
