use std::collections::VecDeque;

/// Inverts each word in given string, while preserving
/// original word order.
/// ```
/// use stringlib::inverse_words;
/// let str = "Hi, I am Pavel.";
/// let result = inverse_words(str);
/// assert_eq!(result, ",iH I ma .levaP");
/// ```
pub fn inverse_words(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    let mut word = VecDeque::<char>::new();
    for ch in str.chars() {
        if ch.is_whitespace() {
            dump_buffer(&mut result, &mut word);
            result.push(ch);
        } else {
            word.push_front(ch);
        }
    }
    dump_buffer(&mut result, &mut word);
    result
}

fn dump_buffer(target: &mut String, word: &mut VecDeque<char>) {
    word.iter().for_each(|c| {
        target.push(c.clone());
    });
    word.clear();
}

#[cfg(test)]
mod tests {
    use super::inverse_words;

    #[test]
    fn test_inverse_str() {
        let str = "Hi, I am Pavel.";
        let result = inverse_words(str);
        assert_eq!(result, ",iH I ma .levaP");
    }

    #[test]
    fn test_inverse_str_end_with_whitespace() {
        let str = "Pavel ";
        let result = inverse_words(str);
        assert_eq!(result, "levaP ");
    }
}
