use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut s = vec![];

    for bit in 0..(1 << 10) {
        let mut sum = 0;
        for i in 0..10 {
            if bit & (1 << i) == 0 {
                sum += 10_usize.pow(i) * 4
            } else {
                sum += 10_usize.pow(i) * 7
            }
        }
        s.push(sum);
    }

    s.sort();

    println!("{}", s[n - 1]);
}
