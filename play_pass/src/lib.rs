trait PasswordShift {
    fn password_shift(&self, iter: usize, shift: u32) -> Self;
    fn shift_letter(&self, shift: u32) -> Self;
    fn digit_complement(&self) -> Self;
    fn shift_case(&self, iter: usize) -> Self;
}

impl PasswordShift for char {
    fn password_shift(&self, iter: usize, shift: u32) -> Self {
        self.shift_letter(shift).digit_complement().shift_case(iter)
    }

    // Shift the ASCII letter by the 'shift' amount (nb: A shifted 1 place becomes B)
    fn shift_letter(&self, shift: u32) -> Self {
        // Shift circularly while keeping the char_digit between the ASCII range of 10~35
        // This is for situations when the provided shift amount is bigger than the total alphabet
        // size, we need to loop back to the beginning of the alphabet
        let shift_ascii_circular = |char_digit, shift| (char_digit + shift - 10) % 26 + 10;

        // Exit early if this char is a number (0-9)
        if self.is_numeric() {
            return *self;
        }
        self.to_digit(36)
            .map(|char_as_digit| {
                std::char::from_digit(shift_ascii_circular(char_as_digit, shift), 36)
                    .unwrap_or(*self)
            })
            .unwrap_or(*self)
    }

    // Returns the complement to 9 for a given digit (as a char)
    // 0123456789 becomes 9876543210
    // To achieve this we just subtract the char.to_digit from 9 and convert back to a char
    fn digit_complement(&self) -> Self {
        self.to_digit(10)
            .map(|i| std::char::from_digit(9 - i, 10).unwrap_or(*self))
            .unwrap_or(*self)
    }

    // Shift the case of the current ASCII char based on it's place in the string.
    // Odd places become lowercase, even places become uppercase
    fn shift_case(&self, iter: usize) -> Self {
        match iter % 2 {
            0 => self.to_ascii_uppercase(),
            1 => self.to_ascii_lowercase(),
            _ => *self,
        }
    }
}

pub fn play_pass(password: &str, shift: u32) -> String {
    password
        .char_indices()
        .map(|(iter, chara)| chara.password_shift(iter, shift))
        .rev()
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalization() {
        assert_eq!(play_pass("I LOVE YOU!!!", 0), "!!!uOy eVoL I"); // Capialization
    }

    #[test]
    fn number_complements() {
        assert_eq!(play_pass("I LOVE YOU 2012!!!", 0), "!!!7897 uOy eVoL I"); // Numbers
    }

    #[test]
    fn ascii_shifting() {
        assert_eq!(play_pass("AAABBCCY", 1), "zDdCcBbB"); // Shifting
    }

    #[test]
    fn circular_shifting() {
        assert_eq!(play_pass("AAABBCCY", 2), "aEeDdCcC"); // Circular shifting
    }

    #[test]
    fn all_at_once() {
        assert_eq!(play_pass("I LOVE YOU 2012Z!!!", 1), "!!!a7897 vPz fWpM J"); // Everything
    }
}
