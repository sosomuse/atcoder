use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut vec = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            if i == 0 || j == 0 || j == i {
                vec[i][j] = 1;
            } else {
                vec[i][j] = vec[i - 1][j - 1] + vec[i - 1][j];
            }

            if i >= j {
                print!("{} ", vec[i][j]);
            }
        }

        println!();
    }
}
