use proconio::input;

fn main() {
    input! {
        (a, b, c, x): (f64,f64,f64,f64),
    }

    let ans = {
        if a >= x {
            1.0
        } else if b < x {
            0.0
        } else {
            c / (b - a)
        }
    };

    println!("{:.12}", ans);
}
