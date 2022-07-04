use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize
    };

    let mut vec = vec![vec![0; n]; 2];

    for (i, (c, p)) in cp.iter().enumerate() {
        if *c == 1 {
            vec[0][i] = vec[0][(i as isize - 1).max(0) as usize] + *p;
            vec[1][i] = vec[1][(i as isize - 1).max(0) as usize];
        } else {
            vec[1][i] = vec[1][(i as isize - 1).max(0) as usize] + *p;
            vec[0][i] = vec[0][(i as isize - 1).max(0) as usize];
        }
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        };

        let ans_1 = vec[0][r - 1] - vec[0][(l as isize - 2).max(0) as usize];
        let ans_2 = vec[1][r - 1] - vec[1][(l as isize - 2).max(0) as usize];

        println!("{} {}", ans_1, ans_2);
    }
}
