pub fn reverse(input: i32) -> i32 {
    // use i64::abs over i32:: since i32::abs(std::i32::MIN) will overflow
    let reversed_int = match i64::abs(input.into())
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
    {
        Ok(v) => v,
        Err(_e) => 0,
    };

    // if the input was negative, negate the return value
    if input < 0 {
        -reversed_int
    } else {
        reversed_int
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(51423), 32415);
    }

    #[test]
    fn negative() {
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(-51423), -32415);
    }

    //std::i32::MIN; -2147483648
    #[test]
    fn underflow() {
        assert_eq!(reverse(-1000000009), 0);
    }

    //std::i32::MAX;  2147483647
    #[test]
    fn overflow() {
        assert_eq!(reverse(1000000009), 0);
    }
}
