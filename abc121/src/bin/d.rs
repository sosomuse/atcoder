use proconio::input;

const MAX: usize = 50;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let sa = solve(if a == 0 { 0 } else { a - 1 });
    let mut sb = solve(b);

    for i in 0..MAX {
        sb[i] -= sa[i];
        sb[i] %= 2;
    }

    let mut ans: usize = 0;
    let mut n: usize = 1;

    for i in 0..MAX {
        if sb[i] == 1 {
            ans += n;
        }

        n *= 2;
    }

    println!("{}", ans);
}

// 引数で渡された数以下の2進数の1の個数を求める
fn solve(b: usize) -> Vec<usize> {
    let mut s = vec![0; MAX];

    let mut n: usize = 1;
    let mut c = 0;

    while n <= b {
        s[c] += b / (n * 2) * n;
        s[c] += ((b % (n * 2)) as isize - n as isize + 1).max(0) as usize;

        c += 1;
        n *= 2;
    }

    s
}
