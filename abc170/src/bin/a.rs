use proconio::input;

fn main() {
    input! {
        x: [usize; 5],
    }

    let mut ans = 0;

    for i in 0..5 {
        let v = x[i];
        if v == 0 {
            ans = i + 1;
            break;
        }
    }

    println!("{}", ans);
}
