use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
        s: String,
    };

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        };

        let x = &s[(a - 1)..b];
        let y = &s[(c - 1)..d];

        if x == y {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
