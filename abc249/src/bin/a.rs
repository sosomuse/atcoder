use proconio::input;

fn main() {
    input! {
        (a, b, c, d, e, f, x): (usize,usize,usize,usize,usize,usize,usize),
    }

    let taka = (x / (a + c)) * a * b + (x % (a + c)).min(a) * b;
    let ao = (x / (d + f)) * d * e + (x % (d + f)).min(d) * e;

    if taka == ao {
        println!("Draw");
    }

    if taka > ao {
        println!("Takahashi");
    }

    if taka < ao {
        println!("Aoki");
    }
}
