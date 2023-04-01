use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(isize, isize); n],
    }

    let (mut min_sum, mut max_sum, mut min_diff, mut max_diff) =
        (1isize << 60, -(1isize << 60), 1isize << 60, -(1isize << 60));

    for (x, y) in points {
        let sum = x + y;
        let diff = x - y;
        min_sum = min_sum.min(sum);
        max_sum = max_sum.max(sum);
        min_diff = min_diff.min(diff);
        max_diff = max_diff.max(diff);
    }

    let ans = (max_sum - min_sum).max(max_diff - min_diff);
    println!("{}", ans);
}
