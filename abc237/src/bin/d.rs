use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut ans = String::from("0");

    for v in 0..n {
        let c = s.chars().nth(v).unwrap();
        let vs = v.to_string();
        let len = vs.len();
        let t = ans.find(&vs).unwrap();

        if c == 'L' {
            let rep = format!("{} {}", v + 1, v);
            eprintln!("{}", rep);
            ans.replace_range(t..t + len, &rep);
        } else if c == 'R' {
            let rep = format!("{} {}", v, v + 1);
            eprintln!("{}", rep);
            ans.replace_range(t..t + len, &rep);
        }
    }

    println!("{}", ans);
}
