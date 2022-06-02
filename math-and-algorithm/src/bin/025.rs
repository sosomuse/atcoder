use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f32; n],
        b: [f32; n],
    }

    let a_sum = a.iter().sum::<f32>();
    let b_sum = b.iter().sum::<f32>();

    let ans = a_sum / 3. + b_sum * 2. / 3.;

    println!("{}", ans)
}
