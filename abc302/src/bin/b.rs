use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let dx = [0, 1, 1, 1, 0, -1, -1, -1];
    let dy = [1, 1, 0, -1, -1, -1, 0, 1];
    let word = ['s', 'n', 'u', 'k', 'e'];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 's' {
                for dir in 0..8 {
                    let mut flag = true;
                    let mut positions = vec![];
                    for k in 0..5 {
                        let ni = i as i32 + k as i32 * dx[dir];
                        let nj = j as i32 + k as i32 * dy[dir];
                        if ni < 0
                            || nj < 0
                            || ni >= h as i32
                            || nj >= w as i32
                            || s[ni as usize][nj as usize] != word[k]
                        {
                            flag = false;
                            break;
                        }
                        positions.push((ni + 1, nj + 1));
                    }
                    if flag {
                        for (ri, ci) in positions {
                            println!("{} {}", ri, ci);
                        }
                        return;
                    }
                }
            }
        }
    }
}
