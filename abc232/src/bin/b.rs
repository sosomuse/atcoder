use proconio::input;
fn main() {
    input! {
        s: String,
        t: String
    }

    let s_bytes: Vec<u8> = s.bytes().map(|b| b - 'a' as u8).collect();
    let t_bytes: Vec<u8> = t.bytes().map(|b| b - 'a' as u8).collect();

    for i in 0..26 {
        let count = s_bytes
            .iter()
            .zip(t_bytes.iter())
            .filter(|(x, y)| (*x + i) % 26 == **y)
            .count();
        if count == s_bytes.len() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
