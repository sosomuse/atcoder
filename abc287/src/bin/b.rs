use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let mut matches = vec![false; n];

    for i in 0..n {
        let v = &s[i];
        let pref = &v[3..];

        let is_match = t.iter().any(|t| t == pref);

        if is_match {
            matches[i] = true;
        }
    }

    println!("{}", matches.iter().filter(|&s| *s).count());
}
