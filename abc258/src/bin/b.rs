use proconio::input;

fn main() {
    input! {
        n: isize,
    };

    let mut a = vec![vec![0; n as usize]; n as usize];
    let mut ans = 0;

    for i in 0..n {
        input! {
            ai: usize,
        };

        let ais = ai.to_string();

        for (j, t) in ais.chars().enumerate() {
            a[i as usize][j] = t.to_digit(10).unwrap() as isize;
        }
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..8 {
                let (x, y) = {
                    match k {
                        0 => (-1, 0),
                        1 => (-1, -1),
                        2 => (0, -1),
                        3 => (1, -1),
                        4 => (1, 0),
                        5 => (1, 1),
                        6 => (0, 1),
                        7 => (-1, 1),
                        _ => unreachable!(),
                    }
                };

                let mut tmp = 0;

                for t in 0..n {
                    let x2 = {
                        let m = i + x * t;
                        if m >= 0 {
                            m % n
                        } else {
                            n - (m.abs() % n)
                        }
                    };
                    let y2 = {
                        let m = j + y * t;
                        if m >= 0 {
                            m % n
                        } else {
                            n - (m.abs() % n)
                        }
                    };
                    let count = a[x2 as usize][y2 as usize];
                    tmp += 10_i64.pow((n - t - 1) as u32) * count as i64;
                }

                ans = ans.max(tmp);
            }
        }
    }

    println!("{}", ans);
}
