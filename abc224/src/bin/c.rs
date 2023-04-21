use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    };

    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = xy[i];
                let (x2, y2) = xy[j];
                let (x3, y3) = xy[k];
                let a = (x2 - x1) * (y3 - y1);
                let b = (x3 - x1) * (y2 - y1);
                if a != b {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
