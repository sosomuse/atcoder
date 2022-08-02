use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut count = 0;
    let mut vec: Vec<usize> = vec![];

    for _ in 0..q {
        input! {
            n: usize,
        };

        if n == 1 {
            input! {
                x: usize,
                c: usize,
            };
        } else {
            input! {
                c: usize,
            }
        }
    }
}
