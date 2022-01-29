use proconio::input;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }

    let mut result: u64 = 0;
    let mut max_count: u64 = 0;

    for i in a {
        if max_count < i {
            max_count = i;
        } else if max_count > i {
            result += max_count - i;
        }
    }

    println!("{}", result);
}
