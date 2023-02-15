use proconio::input;

fn main() {
    input! {
        n: usize,
        lrxy: [(usize, usize, usize, usize); n],
    };

    // 二次元累積和（imos法）
    let mut s = vec![vec![0isize; 1010]; 1010];

    for (lx, ly, rx, ry) in lrxy {
        s[lx][ly] += 1;
        s[lx][ry] -= 1;
        s[rx][ly] -= 1;
        s[rx][ry] += 1;
    }

    for i in 0..1001 {
        for j in 0..1001 {
            s[i][j + 1] += s[i][j];
        }
    }

    for i in 0..1001 {
        for j in 0..1001 {
            s[i + 1][j] += s[i][j];
        }
    }

    let mut ans = vec![0; n + 1];
    for i in 0..1001 {
        for j in 0..1001 {
            ans[s[i][j] as usize] += 1;
        }
    }

    for i in 1..=n {
        println!("{}", ans[i]);
    }
}
