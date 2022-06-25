use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    };

    let mut adults = vec![];
    let mut children = vec![];
    let mut child_max = 0;
    let mut adult_min = 10000000000;

    for i in 0..n {
        let v = w[i];
        if s[i] == '1' {
            adult_min = adult_min.min(v);
            adults.push(v);
        } else {
            child_max = child_max.max(v);
            children.push(v);
        }
    }

    if children.len() == n || adults.len() == n {
        println!("{}", n);
        return;
    }

    let solve = |x: usize| {
        let mut t = 0;

        for i in 0..n {
            let v = w[i];

            if s[i] == '1' {
                if v >= x {
                    t += 1;
                }
            } else {
                if v < x {
                    t += 1;
                }
            }
        }

        t
    };

    let mut ans = 0;

    if adults.len() > children.len() {
        for v in adults {
            ans = ans.max(solve(v));
        }
    } else {
        for v in children {
            ans = ans.max(solve(v));
        }
    }

    println!("{}", ans);
}
