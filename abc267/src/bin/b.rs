use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s[0] == '1' {
        println!("No");
        return;
    }

    let mut vec = vec![true; 7];

    // 6,
    // 3,
    // 1, 7,
    // 0, 4,
    // 2, 8,
    // 5,
    // 9,

    if s[6] == '0' {
        vec[0] = false;
    };

    if s[3] == '0' {
        vec[1] = false;
    };

    if s[1] == '0' && s[7] == '0' {
        vec[2] = false;
    };

    if s[0] == '0' && s[4] == '0' {
        vec[3] = false;
    };

    if s[2] == '0' && s[8] == '0' {
        vec[4] = false;
    };

    if s[5] == '0' {
        vec[5] = false;
    };

    if s[9] == '0' {
        vec[6] = false;
    };

    let mut count = 0;

    for i in 0..7 {
        let x = vec[i];

        if count == 0 && x {
            count += 1;
        }

        if count == 1 && !x {
            count += 1;
        }

        if count == 2 && x {
            count += 1;
        }
    }

    if count == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
