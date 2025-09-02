fn calculate_hash_for_char(initial_value: u64, c: char) -> u64 {
    (initial_value + (c as u64)) * 17 % 256
}

pub fn calculate_hash(str: &str) -> u64 {
    str.chars()
        .fold(0, |result, c| calculate_hash_for_char(result, c))
}
