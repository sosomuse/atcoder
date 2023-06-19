use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = vec![vec![0; n]; n];
    let mut first = 0;

    for i in 0..n {
        let vec = vec![0, 1, 2];

        for v in vec {
            if first + v < n {
                ans[i][first + v] = 1;
            } else {
                ans[i][first + v - n] = 1;
            }
        }

        first += 3;
        if first >= n {
            first -= n;
        }
    }

    if n % 3 != 0 {
        ans.swap(0, n / 3 - 1);
        ans.swap(n - 1, n - n / 3);
    }

    for i in 0..n {
        for j in 0..n {
            if ans[i][j] == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
