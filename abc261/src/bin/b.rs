use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let t = a[i][j];

            match t {
                'W' => {
                    if a[j][i] != 'L' {
                        println!("incorrect");
                        return;
                    }
                }
                'L' => {
                    if a[j][i] != 'W' {
                        println!("incorrect");
                        return;
                    }
                }
                'D' => {
                    if a[j][i] != 'D' {
                        println!("incorrect");
                        return;
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    println!("correct");
}
