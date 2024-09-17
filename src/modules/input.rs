use std::io::stdin;

pub fn input() -> i64 {
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}
