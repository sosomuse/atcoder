use proconio::input;

fn main() {
    input! {
        n: i32,
        sum: i32,
    }
    for x in 0..=n {
        for y in 0..=n {
            let z = n - x - y;
            if z >= 0 && x * 10000 + y * 5000 + z * 1000 == sum {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
