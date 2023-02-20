use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    };

    let mut c = vec![vec![0usize; 61]; 61];

    c[0][0] = 1;

    for i in 0..60 {
        for j in 0..i + 1 {
            c[i + 1][j] += c[i][j];
            c[i + 1][j + 1] += c[i][j];
        }
    }

    let mut ans = String::new();

    while a + b > 0 {
        let mut x = 0;

        if a >= 1 {
            x = c[a + b - 1][a - 1];
        }

        if k <= x {
            ans.push('a');
            a -= 1;
        } else {
            ans.push('b');
            b -= 1;
            k -= x;
        }
    }

    println!("{}", ans);
}
