use proconio::input;

fn main() {
    input!(s: String);

    let mut end_idx: usize = s.len();

    loop {
        for i in 5..=7 {
            if end_idx < i {
                println!("NO");
                return;
            }
            if &s[(end_idx - i)..end_idx] == "dream"
                || &s[(end_idx - i)..end_idx] == "dreamer"
                || &s[(end_idx - i)..end_idx] == "erase"
                || &s[(end_idx - i)..end_idx] == "eraser"
            {
                end_idx -= i;
                break;
            }
            if i == 7 {
                println!("NO");
                return;
            }
        }
        if end_idx == 0 {
            println!("YES");
            break;
        }
    }
}
