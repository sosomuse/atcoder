use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };

    for i in a..=b {
        let mut tmp = false;

        for j in c..=d {
            if is_prime(i + j) {
                tmp = true;
            }
        }

        if !tmp {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
}

fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}
