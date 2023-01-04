use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let x = (2. * b / a).powf(-2.0 / 3.0) - 1.0;
    if x < 0.0 {
        println!("{}", a);
    } else {
        let mut c = (x as i64) as f64;
        let mut ans = b * c + a * (1.0 + c).powf(-1.0 / 2.0);
        c += 1.0;
        ans = ans.min(b * c + a * (1.0 + c).powf(-1.0 / 2.0));
        println!("{}", ans);
    }
}
