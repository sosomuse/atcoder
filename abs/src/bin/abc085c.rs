use proconio::input;

fn main() {
    input! {
        (n, y): (i32, i32),
    }

    for i in 0..=n {
        for j in 0..=n {
            let c = n - i - j;

            if i + j > n {
                continue;
            }

            if i * 10000 + j * 5000 + c * 1000 == y {
                println!("{} {} {}", i, j, c);
                std::process::exit(0);
            }
        }
    }

    println!("-1 -1 -1");
}
