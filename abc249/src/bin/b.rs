use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut exist_upper = false;
    let mut exist_lower = false;
    let mut once = true;

    let chars = s.chars().collect::<Vec<char>>();

    for (i, v) in chars.iter().enumerate() {
        if v.is_ascii_uppercase() {
            exist_upper = true;
        } else if v.is_ascii_lowercase() {
            exist_lower = true;
        }

        for (j, k) in chars.iter().enumerate() {
            if i == j {
                continue;
            }

            if v == k {
                once = false;
            }
        }
    }

    if exist_upper && exist_lower && once {
        println!("Yes");
    } else {
        println!("No");
    }
}
