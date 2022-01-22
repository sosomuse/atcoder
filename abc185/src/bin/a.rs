use proconio::input;

fn main() {
    input! {
        a: [i32; 4],
    }
    let mut r: i32 = 0;
    for i in a {
        if r == 0 {
            r = i
        } else {
            r = r.min(i)
        }
    }
    println!("{}", r);
}
