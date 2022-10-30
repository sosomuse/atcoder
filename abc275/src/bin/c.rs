use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    };

    let mut ans = 0;

    let vec = vec![
        (1, 2),
        (2, 1),
        (1, 4),
        (4, 1),
        (1, 6),
        (6, 1),
        (1, 8),
        (8, 1),
    ];

    for i in 0..9 {
        for j in 0..9 {
            let v = s[i][j];

            if v != '#' {
                continue;
            }

            for j2 in j + 1..9 {
                let k = j2 - j;

                if s[i][j2] == '#' {
                    let i2 = i + k;
                    let j2 = j + k;

                    if j2 >= 9 || i2 >= 9 {
                        break;
                    }

                    if s[i2][j] == '#' && s[i2][j2] == '#' {
                        ans += 1;
                    }
                }
            }

            for t in i + 1..9 {
                let k = t - i;
                let i2 = i + k;
                let j2 = j + k;

                if j2 >= 9 || i2 >= 9 {
                    break;
                }

                if s[i2][j2] == '#' {
                    if i2 >= k * 2 && j2 + k < 9 {
                        if s[i2 - k * 2][j2] == '#' && s[i2 - k][j2 + k] == '#' {
                            ans += 1;
                        }
                    }
                }
            }

            for (v1, v2) in vec.iter() {
                let i2 = i + v1;
                let j2 = j + v2;

                if i2 >= 9 || j2 >= 9 {
                    continue;
                }

                if s[i2][j2] == '#' {
                    if j < *v2 || i + v1 >= 9 {
                        continue;
                    }

                    let i3 = i + *v1;
                    let j3 = j - *v2;

                    if s[i3][j3] == '#' {
                        dbg!(i, j, i2, j2, i3, j3);
                        if j < *v1 || j + *v2 >= 9 {
                            continue;
                        }

                        let i4 = i - v1;
                        let j4 = j + v2;

                        if s[i4][j4] == '#' {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
