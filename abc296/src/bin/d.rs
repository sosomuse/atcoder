use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for i in m..=1000000000000 {
        if n * n < i {
            println!("-1");
            return;
        }

        let mut a = i / n;
        if a == 0 {
            a = 1;
        }
        while a * a <= i {
            if i % a == 0 {
                if a <= n && i / a <= n {
                    println!("{}", i);
                    return;
                }
            }

            a += 1;
        }
    }
}
