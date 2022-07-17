use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };

    if n == 1 {
        println!("{}", 0);
        return;
    }

    let mut red = vec![0; n + 1];
    let mut blue = vec![0; n + 1];

    red[n] = 1;

    loop {
        let mut tmp = false;
        for i in 0..n + 1 {
            if red[i] >= 1 {
                if i == 1 {
                    continue;
                }

                tmp = true;

                let v = red[i];
                red[i] = 0;
                red[i - 1] += v;
                blue[i] += v * x;
            }
        }

        for i in 0..n + 1 {
            if blue[i] > 1 {
                if i == 1 {
                    continue;
                }

                tmp = true;

                let v = blue[i];
                blue[i] = 0;
                blue[i - 1] += v * y;
                red[i - 1] += v;
            }
        }

        if !tmp {
            break;
        }
    }

    println!("{}", blue[1]);
}
