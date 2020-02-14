pub fn create_phone_number(numbers: &[u8]) -> String {
    let numbers = numbers
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("");
    format!("({}) {}-{}", &numbers[..3], &numbers[3..6], &numbers[6..],)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }
}
