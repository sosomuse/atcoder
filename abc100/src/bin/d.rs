use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(isize, isize, isize); n],
    };

    let mut ans = 0;

    for i in 0..1 << 3 {
        let mut coefficients = vec![0, 0, 0];

        for j in 0..3 {
            if i & (1 << j) != 0 {
                coefficients[j] = 1;
            } else {
                coefficients[j] = -1;
            }
        }

        let mut s = xyz
            .iter()
            .map(|(x, y, z)| x * coefficients[0] + y * coefficients[1] + z * coefficients[2])
            .collect::<Vec<_>>();
        s.sort_by(|a, b| b.cmp(a));

        let tmp = s.iter().take(m).sum::<isize>();
        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
