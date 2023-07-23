use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }

    let mut free_days = vec![true; d];
    for schedule in &s {
        for (j, &day) in schedule.iter().enumerate() {
            if day == 'x' {
                free_days[j] = false;
            }
        }
    }

    let mut max_free_days = 0;
    let mut current_free_days = 0;
    for &day in &free_days {
        if day {
            current_free_days += 1;
            max_free_days = max_free_days.max(current_free_days);
        } else {
            current_free_days = 0;
        }
    }

    println!("{}", max_free_days);
}
