use proconio::input;

fn main() {
    input! {
        s: [char; 3],
        t: [char; 3]
    }

    let mut cnt = 0;

    for i in 0..3 {
        if s[i] != t[i] {
            cnt += 1;
        }
    }

    if cnt > 0 && cnt % 2 == 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
