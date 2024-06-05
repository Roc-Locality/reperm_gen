
pub fn factorial(num: i128) -> i128 {
    (1..=num).product()
}

pub fn combinations(n: i128, k: i128) -> i128 {
    (factorial(n) / factorial(k)) / factorial(n - k) 
}