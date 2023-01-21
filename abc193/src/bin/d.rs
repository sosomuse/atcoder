use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        k: usize,
        mut s: String,
        mut t: String,
    };

    let mut set: HashSet<(usize, usize)> = HashSet::new();
    s.remove(4);
    t.remove(4);

    for i in 1..=9 {
        for j in 1..=9 {
            let mut s1 = s.clone();
            s1.push_str(&i.to_string());
            let mut t1 = t.clone();

            t1.push_str(&j.to_string());

            let s_score = get_score(s1);
            let t_score = get_score(t1);

            if s_score > t_score {
                set.insert((i, j));
            }
        }
    }

    let mut count = vec![k; 10];

    for c in s.chars() {
        count[c.to_digit(10).unwrap() as usize] -= 1;
    }
    for c in t.chars() {
        count[c.to_digit(10).unwrap() as usize] -= 1;
    }

    let mut sum = 0;
    for i in 1..=9 {
        sum += count[i];
    }
    let parent = sum * (sum - 1);
    let mut child = 0;

    for i in 1..=9 {
        for j in 1..=9 {
            if set.contains(&(i, j)) {
                if count[i] == 0 || count[j] == 0 {
                    continue;
                }

                if i == j {
                    child += count[i] * (count[j] - 1);
                } else {
                    child += count[i] * count[j];
                }
            }
        }
    }

    println!("{}", child as f64 / parent as f64);
}

fn get_score(s: String) -> usize {
    let mut count = vec![0; 10];
    let mut score = 0;

    for c in s.chars() {
        count[c.to_digit(10).unwrap() as usize] += 1;
    }

    for i in 1..=9 {
        score += i * 10usize.pow(count[i] as u32);
    }

    score
}
