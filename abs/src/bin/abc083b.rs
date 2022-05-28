use proconio::input;

fn main() {
    input! {
        (n, a, b): (u32, u32, u32),
    }

    let mut vec: Vec<u32> = Vec::new();

    for v in 0..=n {
        if v.to_string().len() == 1 && v >= a && v <= b {
            vec.push(v);
        } else {
            let s = v.to_string();
            let mut sum = 0;
            for c in s.chars() {
                sum += c.to_digit(10).unwrap();
            }
            if sum >= a && sum <= b {
                vec.push(v);
            }
        }
    }

    dbg!(&vec);

    let ans = vec.iter().sum::<u32>();

    print!("{}", ans);
}
