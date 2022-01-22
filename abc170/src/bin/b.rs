use proconio::input;

// 鶴足2本, 亀足4本
// Yes, No
fn main() {
    input! {
        x: i32,
        y: i32,
    }
    if x * 4 < y || x * 2 > y {
        println!("No");
        return;
    }

    let a = y % 4;

    if a == 0 || a == 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
