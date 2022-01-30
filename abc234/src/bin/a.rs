use proconio::input;

fn main() {
    input! {
        t: u32,
    }

    println!("{}", f(f(f(t) + t) + f(f(t))));
}

fn f(t: u32) -> u32 {
    t * t + 2 * t + 3
}
