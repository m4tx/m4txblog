use chrono::Datelike;

pub(crate) fn current_year() -> i32 {
    chrono::Utc::now().year()
}

/// Splits off the last word of `text`, so that it can be kept on the same line
/// as an element (such as an icon) that follows it.
///
/// The first returned part keeps its trailing whitespace.
pub(crate) fn split_last_word(text: &str) -> (&str, &str) {
    match text.rfind(char::is_whitespace) {
        Some(index) => {
            let whitespace_len = text[index..].chars().next().map_or(0, char::len_utf8);
            text.split_at(index + whitespace_len)
        }
        None => ("", text),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_last_word_multiple_words() {
        assert_eq!(
            split_last_word("Rust Web Development"),
            ("Rust Web ", "Development")
        );
    }

    #[test]
    fn split_last_word_single_word() {
        assert_eq!(split_last_word("Rust"), ("", "Rust"));
    }

    #[test]
    fn split_last_word_empty() {
        assert_eq!(split_last_word(""), ("", ""));
    }
}
