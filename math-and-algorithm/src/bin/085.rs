use proconio::input;

fn main() {
    input! {
        n: isize,
        x: isize,
        y: isize,
    };

    for i in 1..=n {
        for j in i..=n {
            for k in j..=n {
                for l in k..=n {
                    if i + j + k + l == x && i * j * k * l == y {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
