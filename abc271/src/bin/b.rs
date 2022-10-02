use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut vec: Vec<Vec<usize>> = vec![];

    for _ in 0..n {
        input! {
            l: usize,
            a: [usize; l],
        };

        vec.push(a);
    }

    for _ in 0..q {
        input! {
            s: usize,
            t: usize,
        };

        println!("{}", vec[s - 1][t - 1]);
    }
}
