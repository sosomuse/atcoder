use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 9],
    };

    let mut ans = 0;

    for i1 in 0..9 {
        for j1 in 0..9 {
            if s[i1][j1] == '#' {
                for i2 in i1..9 {
                    for j2 in j1 + 1..9 {
                        if s[i2][j2] == '#' {
                            let x = i2 - i1;
                            let y = j2 - j1;

                            if j2 < x || j1 < x {
                                continue;
                            }

                            let i3 = i2 + y;
                            let j3 = j2 - x;
                            let i4 = i1 + y;
                            let j4 = j1 - x;

                            if i3 >= 9 || i4 >= 9 {
                                continue;
                            }

                            if s[i3][j3] == '#' && s[i4][j4] == '#' {
                                ans += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
