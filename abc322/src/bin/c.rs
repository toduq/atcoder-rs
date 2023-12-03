use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }
    a.reverse();

    let mut results = vec![];
    for d in 1..=n {
        let result = a.last().unwrap() - d;
        results.push(result.to_string());
        if result == 0 {
            a.pop();
        }
    }
    println!("{}", results.join("\n"));
}
