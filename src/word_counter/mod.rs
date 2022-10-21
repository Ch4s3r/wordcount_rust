use regex::Regex;

pub struct WordCounter {
    valid_word_regex: Regex,
}

impl Default for WordCounter {
    fn default() -> Self {
        WordCounter {
            valid_word_regex: Regex::new(r"[a-zA-Z]").unwrap(),
        }
    }
}

impl WordCounter {
    pub fn count(&self, input: &str) -> u8 {
        let mut count = 0;
        for word in input.split_ascii_whitespace() {
            if self.valid_word_regex.is_match(word) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Mary had a little lamb";
        let actual = WordCounter::default().count(input);
        assert_eq!(actual, 5);
    }
}