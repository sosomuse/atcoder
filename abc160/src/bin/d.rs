use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        y: isize,
    };

    let mut ans = vec![0; n - 1];
    for i in 1..=n {
        for j in i + 1..=n {
            let d = std::cmp::min(
                j - i,
                ((x - i as isize).abs() + 1 + (y - j as isize).abs()) as usize,
            );
            ans[d - 1] += 1;
        }
    }

    for i in 0..n - 1 {
        println!("{}", ans[i]);
    }
}
