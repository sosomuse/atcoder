use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    };

    let mut sums = p
        .iter()
        .map(|v| v.iter().sum::<usize>())
        .collect::<Vec<_>>();
    sums.sort_by(|a, b| b.cmp(a));
    let x = sums[k - 1];

    for i in 0..n {
        let v = p[i].iter().sum::<usize>() + 300;

        if v >= x {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
