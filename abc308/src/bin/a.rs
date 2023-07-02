use proconio::input;

fn main() {
    input! {
        s: [usize; 8],
    }
    let mut prev = s[0];
    let mut ok = true;

    if prev < 100 || prev > 675 || prev % 25 != 0 {
        ok = false;
    }

    for i in 1..8 {
        if s[i] < prev || s[i] > 675 || s[i] % 25 != 0 {
            ok = false;
        }
        prev = s[i];
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
