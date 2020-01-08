
struct Password {
    chars: Vec<char>,
}

impl Password {
    fn new(chars: Vec<char>) -> Password {
        Password { chars: chars }
    }

    fn parse<S: Into<String>>(string: S) -> Password {
        Password::new(string.into().chars().collect())
    }

    fn valid(self) -> bool {
        self.length_validator() && 
            self.digit_validator() && 
            self.increase_validator() &&
            self.double_validator()
    }

    fn part2_valid(self) -> bool {
        self.length_validator() && 
            self.digit_validator() && 
            self.increase_validator() &&
            self.part2_double_validator()
    }

    fn length_validator(&self) -> bool {
        self.chars.len() == 6
    }

    fn digit_validator(&self) -> bool {
        self.chars.iter().all(|&c| c.is_digit(10))
    }

    fn increase_validator(&self) -> bool {
        let mut prev = 0u32;
        let mut cur;
        for d in self.chars.iter() {
            cur = d.to_digit(10).unwrap();
            if prev > cur {
                return false;
            }
            prev = cur;
        }
        true
    }
    
    fn double_validator(&self) -> bool {
        let mut prev;
        let mut cur;
        let mut i = 1;
        while i < self.chars.len() {
            prev = self.chars[i-1];
            cur = self.chars[i];
            if prev == cur {
                return true;
            }
            i += 1;
        } 
        false
    }
    
    fn part2_double_validator(&self) -> bool {
        let mut prev;
        let mut cur;
        let mut repeat_counter = 0;
        let mut i = 1;
        while i < self.chars.len() {
            prev = self.chars[i-1];
            cur = self.chars[i];
            if prev == cur {
                repeat_counter += 1;
            } else {
                if repeat_counter == 1 {
                    return true;
                }
                repeat_counter = 0;
            }
            i += 1;
        } 
        repeat_counter == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_one() {
        // password must be 6 digits
        let p1 = Password::parse("11111");
        assert_eq!(p1.valid(), false);
        
        // password must contain only digits
        let p1 = Password::parse("11111b");
        assert_eq!(p1.valid(), false);
        
        // password must contain only digits
        let p1 = Password::parse("223450");
        assert_eq!(p1.valid(), false);
        
        // password contains double digits
        let p1 = Password::parse("123789");
        assert_eq!(p1.valid(), false);
        
        // valid password
        let p1 = Password::parse("111111");
        assert_eq!(p1.valid(), true);
    }

    #[test]
    fn solution1() {
        let range = 402328..864247;
        let mut valid_count = 0;
        for password in range {
            let p = Password::parse(password.to_string());
            if p.valid() {
                valid_count += 1;
            }
        }
        assert_eq!(valid_count, 454);
    }
    
    #[test]
    fn test_two() {
        // double digits must only be doubles
        let p1 = Password::parse("123444");
        assert_eq!(p1.part2_valid(), false);
        
        // valid password
        let p1 = Password::parse("112233");
        assert_eq!(p1.part2_valid(), true);
        
        // valid password
        let p1 = Password::parse("111122");
        assert_eq!(p1.part2_valid(), true);
    }
    
    #[test]
    fn solution2() {
        let range = 402328..864247;
        let mut valid_count = 0;
        for password in range {
            let p = Password::parse(password.to_string());
            if p.part2_valid() {
                valid_count += 1;
            }
        }
        assert_eq!(valid_count, 288);
    }
}
