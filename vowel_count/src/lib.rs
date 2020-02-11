pub fn get_count(string: &str) -> usize {
    string.chars().filter(|&c| "aeiou".contains(c)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_count("abracadabra"), 5);
    }
}
