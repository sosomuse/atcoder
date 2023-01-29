use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };

    let mut c = s.clone();
    c.sort();

    for v in s {
        let current_i = c.binary_search(&v).unwrap();
        let prev_i = (current_i as isize - 1).max(0) as usize;
        let next_i = (current_i + 1).min(n - 1);

        let mut prev_count = 0;
        let mut next_count = 0;
        let current = &c[current_i];
        let prev = &c[prev_i];
        let next = &c[next_i];

        if current_i != prev_i {
            for i in 0..current.len() {
                if i >= prev.len() {
                    break;
                }

                if current[i..i + 1] == prev[i..i + 1] {
                    prev_count += 1;
                } else {
                    break;
                }
            }
        }

        if current_i != next_i {
            for i in 0..current.len() {
                if i >= next.len() {
                    break;
                }

                if current[i..i + 1] == next[i..i + 1] {
                    next_count += 1;
                } else {
                    break;
                }
            }
        }

        let ans = prev_count.max(next_count);

        println!("{}", ans);
    }
}
