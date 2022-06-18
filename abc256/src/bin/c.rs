use proconio::input;

fn main() {
    input! {
        h1: usize,
        h2: usize,
        h3: usize,
        w1: usize,
        w2: usize,
        w3: usize,
    }

    let mut count = 0;

    for v1 in 1..=h1 - 2 {
        for v2 in 1..=h1 - v1 {
            let v3 = h1 - v1 - v2;

            if v3 == 0 {
                continue;
            }

            for v4 in 1..=h2 - 2 {
                for v5 in 1..=h2 - v4 {
                    let v6 = h2 - v4 - v5;

                    if v6 == 0 {
                        continue;
                    }

                    for v7 in 1..=h3 - 2 {
                        for v8 in 1..=h3 - v7 {
                            let v9 = h3 - v7 - v8;

                            if v9 == 0 {
                                continue;
                            }

                            if v1 + v2 + v3 == h1 && v4 + v5 + v6 == h2 && v7 + v8 + v9 == h3 {
                                if v1 + v4 + v7 == w1 && v2 + v5 + v8 == w2 && v3 + v6 + v9 == w3 {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
