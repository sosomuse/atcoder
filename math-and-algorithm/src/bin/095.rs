use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize
    };

    let mut vec = vec![vec![0; n + 1]; 2];

    for (i, (c, p)) in cp.iter().enumerate() {
        if *c == 1 {
            vec[0][i + 1] = vec[0][i] + *p;
            vec[1][i + 1] = vec[1][i];
        } else {
            vec[1][i + 1] = vec[1][i] + *p;
            vec[0][i + 1] = vec[0][i];
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        let ans_1 = vec[0][r] - vec[0][(l - 1)];
        let ans_2 = vec[1][r] - vec[1][(l - 1)];

        println!("{} {}", ans_1, ans_2);
    }
}
