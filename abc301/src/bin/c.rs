use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut s_counts = vec![0; 26];
    let mut s_joker_count = 0;
    let mut t_counts = vec![0; 26];
    let mut t_joker_count = 0;

    for c in s {
        if c == '@' {
            s_joker_count += 1;
        } else {
            s_counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    for c in t {
        if c == '@' {
            t_joker_count += 1;
        } else {
            t_counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    for i in 0..26 {
        if (i + b'a') as char == 'a'
            || (i + b'a') as char == 't'
            || (i + b'a') as char == 'c'
            || (i + b'a') as char == 'o'
            || (i + b'a') as char == 'd'
            || (i + b'a') as char == 'e'
            || (i + b'a') as char == 'r'
        {
            let i: usize = i as usize;

            if s_counts[i] > t_counts[i] {
                if s_counts[i] - t_counts[i] > t_joker_count {
                    println!("No");
                    return;
                }

                t_joker_count -= s_counts[i] - t_counts[i];
            } else {
                if t_counts[i] - s_counts[i] > s_joker_count {
                    println!("No");
                    return;
                }

                s_joker_count -= t_counts[i] - s_counts[i];
            }

            continue;
        }

        let i: usize = i as usize;

        if s_counts[i] != t_counts[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
