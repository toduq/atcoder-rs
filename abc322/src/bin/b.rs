use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String,
    }

    let result = match (t.starts_with(&s), t.ends_with(&s)) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    println!("{}", result);
}
