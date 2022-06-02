use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [f32; n],
        r: [f32; n],
    }

    let b_ave = b.iter().sum::<f32>() / b.len() as f32;
    let r_ave = r.iter().sum::<f32>() / r.len() as f32;

    println!("{}", r_ave + b_ave)
}
