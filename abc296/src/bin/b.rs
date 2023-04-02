use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        board: [Chars; 8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if board[i][j] == '*' {
                let row = (8 - i).to_string();
                let col = (b'a' + j as u8) as char;
                println!("{}{}", col, row);
                return;
            }
        }
    }
}
