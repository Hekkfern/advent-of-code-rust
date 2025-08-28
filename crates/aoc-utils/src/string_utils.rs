/// Extracts only the first line
pub fn extract_first_line(s: &str) -> String {
    s.trim().lines().next().unwrap_or("").to_string()
}

/// Extracts the content of a string as a list of strings separated by new lines
pub fn convert_to_list_of_strings(s: &str) -> Vec<String> {
    s.trim()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

/// Extracts the content of a string as a list of numbers separated by new lines
pub fn convert_to_list_of_numbers<T>(s: &str) -> Vec<T>
where
    T: std::str::FromStr + num_integer::Integer,
{
    s.trim()
        .lines()
        .filter_map(|line| line.trim().parse::<T>().ok())
        .collect()
}

/// Extracts the content of a string as groups of numbers, separated by empty lines
pub fn convert_to_groups_of_numbers<T>(s: &str) -> Vec<Vec<T>>
where
    T: std::str::FromStr + num_integer::Integer,
{
    s.trim()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .filter_map(|line| line.trim().parse::<T>().ok())
                .collect()
        })
        .filter(|group: &Vec<T>| !group.is_empty())
        .collect()
}

/// Extracts the content of a string as a list of numbers, where each character of a line is an independent digit
pub fn convert_to_matrix_of_digits(s: &str) -> Vec<Vec<u8>> {
    s.trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

/// Extracts the content of a string as a matrix of characters.
pub fn convert_to_matrix_of_chars(s: &str) -> Vec<Vec<char>> {
    s.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
