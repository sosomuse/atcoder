use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut vec: Vec<usize> = vec![];

    for i in 0..n {
        vec.push(ab[i].1);
        vec.push(ab[i].0 - ab[i].1);
    }

    vec.sort_by(|a, b| b.cmp(a));

    let sum = vec[0..k].iter().sum::<usize>();

    println!("{}", sum);
}
