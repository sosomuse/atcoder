use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut prev = a[0];
    print!("{} ", prev);

    for i in 1..n {
        let c = a[i];

        if c > prev {
            for j in prev + 1..=c {
                print!("{} ", j);
            }
        } else {
            for j in (c..prev).rev() {
                print!("{} ", j);
            }
        }

        prev = c;
    }
}
