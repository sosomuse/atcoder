use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(String, String); n],
    }

    for i in 0..n {
        let mut can_give = true;
        for (j, st) in a.iter().enumerate() {
            if i == j {
                continue;
            }

            let (s1, t1) = &a[i];
            let (s2, t2) = st;

            if (s1 == s2 || s1 == t2) && (t1 == s2 || t1 == t2) {
                can_give = false;
                break;
            }
        }

        if !can_give {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
