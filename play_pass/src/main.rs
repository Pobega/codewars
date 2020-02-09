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

    fn shift_letter(&self, shift: u32) -> Self {
        // Shift circularly while keeping the char_digit between the ASCII range of 10~35
        // This is for situations when the provided shift amount is bigger than the total alphabet
        // size, we need to loop back to the beginning of the alphabet
        let shift_ascii_circular = |char_digit, shift| {
            (char_digit + shift - 10) % 26 + 10
        };

        // Exit early if this char is a number (0-9)
        if self.is_numeric() {
            return *self;
        }
        match self.to_digit(36) {
            Some(char_as_digit) => {
                match std::char::from_digit(shift_ascii_circular(char_as_digit, shift), 36) {
                    Some(c) => c,
                    None => *self,
                }
            }
            None => *self,
        }
    }

    fn digit_complement(&self) -> Self {
        // Returns the complement to 9 for a given digit (as a char)
        // 0123456789 becomes 9876543210
        // To achieve this we just subtract the char.to_digit from 9 and convert back to a char
        match self.to_digit(10) {
            Some(i) => match std::char::from_digit(9 - i, 10) {
                Some(c) => c,
                None => *self,
            },
            None => *self,
        }
    }

    fn shift_case(&self, iter: usize) -> Self {
        match iter % 2 {
            0 => self.to_ascii_uppercase(),
            1 => self.to_ascii_lowercase(),
            _ => *self,
        }
    }
}

fn play_pass(password: &str, shift: u32) -> String {
    password
        .chars()
        .enumerate()
        .map(|(iter, chara)| chara.password_shift(iter, shift))
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
}

fn main() {
    assert_eq!(play_pass("I LOVE YOU!!!", 0), "!!!uOy eVoL I"); // Capialization
    assert_eq!(play_pass("I LOVE YOU 2012!!!", 0), "!!!7897 uOy eVoL I"); // Numbers
    assert_eq!(play_pass("AAABBCCY", 1), "zDdCcBbB"); // Shifting
    assert_eq!(play_pass("AAABBCCY", 2), "aEeDdCcC"); // Circular shifting
    assert_eq!(play_pass("I LOVE YOU 2012Z!!!", 1), "!!!a7897 vPz fWpM J"); // Everything
}
