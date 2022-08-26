use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n*2],
    };

    let mut cost = 0;

    while a.len() != 0 {
        let mut min = 100000000000000;
        let mut min_i = 0;

        for i in 0..a.len() {
            if i + 1 < a.len() && (a[i] - a[i + 1]).abs() < min {
                min = (a[i] - a[i + 1]).abs();
                min_i = i;
            }
        }

        let x = a[min_i];
        let y = a[min_i + 1];

        cost += (x - y).abs();

        a.remove(min_i);
        a.remove(min_i);
    }

    println!("{}", cost);
}
