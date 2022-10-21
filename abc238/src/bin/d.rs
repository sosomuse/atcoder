use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            a: usize,
            s: usize,
        };

        if a * 2 <= s {
            let dif = s - a * 2;
            if (dif & a) == 0 {
                println!("Yes");
                continue;
            }
        }

        println!("No");
    }
}
