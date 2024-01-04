use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let s1 = &s[0..1];
    let s2 = &s[1..2];
    let t1 = &t[0..1];
    let t2 = &t[1..2];

    let s = "ABCDE";

    let mut dist1 = (s.find(s1).unwrap() as i32 - s.find(s2).unwrap() as i32).abs();
    let mut dist2 = (s.find(t1).unwrap() as i32 - s.find(t2).unwrap() as i32).abs();

    if dist1 == 4 {
        dist1 = 1;
    }

    if dist2 == 4 {
        dist2 = 1;
    }

    let ans = (dist1 == 1 && dist2 == 1) || (dist1 != 1 && dist2 != 1);

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
