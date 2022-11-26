use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
    }

    let mut ans = vec![];

    for i in 0..=9999 {
        let str = format!("{:04}", i);
        let mut flag = true;
        for i in 1..=n {
            let (s, t) = &st[i - 1];
            if solve(s, &str) != *t {
                flag = false;
            }
        }

        if flag {
            dbg!(&str);
            ans.push(str);
        }
    }

    if ans.len() != 1 {
        println!("Can't Solve");
    } else {
        println!("{}", ans[0]);
    }
}

fn solve(a1: &str, a2: &str) -> usize {
    let mut diff = 0;

    for i in 0..a1.len() {
        if a1[i..i + 1] != a2[i..i + 1] {
            diff += 1;
        }
    }

    if diff == 0 {
        1
    } else if diff == 1 {
        2
    } else {
        3
    }
}
