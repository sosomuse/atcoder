use proconio::input;

fn main() {
    input! {
        (n, a, b): (u8, u8, u8),
    }

    for i in 0..n {
        for _ in 0..a {
            for k in 0..n {
                for _ in 0..b {
                    if i % 2 == 1 {
                        if k % 2 == 0 {
                            print!("#");
                            continue;
                        } else {
                            print!(".");
                            continue;
                        }
                    }

                    if k % 2 == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                }
            }

            println!();
        }
    }
}
