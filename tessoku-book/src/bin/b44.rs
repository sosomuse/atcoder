use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
    };

    let mut vec = vec![0; n];

    for i in 0..n {
        vec[i] = i;
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            y: usize,
        };

        match t {
            1 => {
                let nx = vec[x - 1];
                let ny = vec[y - 1];
                vec[x - 1] = ny;
                vec[y - 1] = nx;
            }
            2 => {
                println!("{}", a[vec[x - 1]][y - 1]);
            }
            _ => unreachable!(),
        }
    }
}
