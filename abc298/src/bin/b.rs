use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    let mut a = a;
    for _ in 0..4 {
        a = rotate(&a);
        let mut ok = true;

        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    ok = false;
                }
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn rotate(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = matrix.len();
    let mut rotated = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            rotated[j][n - 1 - i] = matrix[i][j];
        }
    }

    rotated
}
