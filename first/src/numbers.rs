pub fn is_even(n: i32) -> bool {
   n % 2 == 0
}

pub fn is_a_valid_number(text: &str) -> bool {
    text.parse::<f64>().is_ok()
}