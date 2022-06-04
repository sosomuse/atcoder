use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    for v in a {
        println!("{} ", v);
    }
}
