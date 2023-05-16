use proconio::input;

fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[isize; 11]; n],
    };

    let mut ans = isize::min_value();

    for bit in 1..1 << 10 {
        let opens = (0..10).map(|i| (bit >> i) & 1).collect::<Vec<_>>();
        let mut tmp = 0;

        for i in 0..n {
            let same_count = (0..10).filter(|j| opens[*j] == 1 && f[i][*j] == 1).count();
            tmp += p[i][same_count];
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
