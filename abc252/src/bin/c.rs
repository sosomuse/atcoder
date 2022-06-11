use proconio::input;

fn main() {
    input! {
        n: u8,
        r: [String; n],
    }

    let mut cnt = vec![vec![0; 10]; 10];

    for s in r.iter() {
        for (j, c) in s.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as usize;
            cnt[c][j] += 1;
        }
    }

    let mut ans = 100000;

    // dbg!(&cnt);

    for i in 0..10 {
        let mut max = 0;
        for j in 0..10 {
            max = max.max(10 * (cnt[i][j] as isize - 1) + j as isize);
        }
        ans = ans.min(max);
    }

    print!("{}", ans);
}
