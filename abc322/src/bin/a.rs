use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut found = -1i32;
    for i in 0..=(n-3) {
        if s[i] == 'A' && s[i+1] == 'B' && s[i+2] == 'C' {
            found = (i + 1) as i32;
            break;
        }
    }
    println!("{}", found);
}
