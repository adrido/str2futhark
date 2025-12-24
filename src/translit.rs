//! Module for transliterating Latin text to Elder Futhark runes.
//!
//! The `transliterate` function performs a mostly direct character-to-rune mapping
//! and supports a few digraphs and common accented/scandinavian letters.
//!
//! Mapping highlights:
//! - Digraphs: `th` -> ᚦ, `ng` -> ᛜ, `ch` -> ᚲ, `sh` -> ᛋ
//! - Supported characters: accented vowels (á, é, í, etc.), `æ`, `ø`, `œ`, `þ`, `ð`.
//!
//! This is a simple transliteration (not phonetic); adjust mappings in this module
//! if you need alternative behavior.

pub fn transliterate(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut iter = input.chars().peekable();
    while let Some(c) = iter.next() {
        let lower = c.to_lowercase().next().unwrap_or(c);

        if lower == 't' {
            if let Some(&next) = iter.peek() {
                if next.to_lowercase().next().unwrap_or(next) == 'h' {
                    out.push_str("ᚦ");
                    iter.next();
                    continue;
                }
            }
        }

        if lower == 'n' {
            if let Some(&next) = iter.peek() {
                if next.to_lowercase().next().unwrap_or(next) == 'g' {
                    out.push_str("ᛜ");
                    iter.next();
                    continue;
                }
            }
        }

        if lower == 's' {
            if let Some(&next) = iter.peek() {
                if next.to_lowercase().next().unwrap_or(next) == 'h' {
                    out.push_str("ᛋ");
                    iter.next();
                    continue;
                }
            }
        }

        if lower == 'c' {
            if let Some(&next) = iter.peek() {
                if next.to_lowercase().next().unwrap_or(next) == 'h' {
                    out.push_str("ᚲ");
                    iter.next();
                    continue;
                }
            }
        }

        match lower {
            'a' | 'á' | 'à' => out.push_str("ᚨ"),
            'b' => out.push_str("ᛒ"),
            'c' => out.push_str("ᚲ"),
            'd' | 'ð' => out.push_str("ᛞ"),
            'e' | 'é' => out.push_str("ᛖ"),
            'f' => out.push_str("ᚠ"),
            'g' => out.push_str("ᚷ"),
            'h' => out.push_str("ᚺ"),
            'i' | 'í' => out.push_str("ᛁ"),
            'j' => out.push_str("ᛃ"),
            'k' => out.push_str("ᚲ"),
            'l' => out.push_str("ᛚ"),
            'm' => out.push_str("ᛗ"),
            'n' => out.push_str("ᚾ"),
            'o' | 'ø' | 'œ' => out.push_str("ᛟ"),
            'p' => out.push_str("ᛈ"),
            'q' => out.push_str("ᚲ"),
            'r' => out.push_str("ᚱ"),
            's' => out.push_str("ᛋ"),
            't' => out.push_str("ᛏ"),
            'u' | 'v' | 'ú' => out.push_str("ᚢ"),
            'w' => out.push_str("ᚹ"),
            'x' => out.push_str("ᛉ"),
            'y' | 'ý' | 'æ' => out.push_str("ᛇ"),
            'z' => out.push_str("ᛉ"),
            'þ' => out.push_str("ᚦ"),
            ' ' => out.push(' '),
            '\n' => out.push('\n'),
            '\t' => out.push('\t'),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::transliterate;

    #[test]
    fn basic_ascii() {
        assert_eq!(transliterate("Hello World"), "ᚺᛖᛚᛚᛟ ᚹᛟᚱᛚᛞ");
    }

    #[test]
    fn digraphs() {
        assert_eq!(transliterate("Thorn"), "ᚦᛟᚱᚾ");
        assert_eq!(transliterate("sing"), "ᛋᛁᛜ");
        assert_eq!(transliterate("church"), "ᚲᚢᚱᚲ");
    }

    #[test]
    fn accented_and_special() {
        assert_eq!(transliterate("Æsir"), "ᛇᛋᛁᚱ");
        assert_eq!(transliterate("það"), "ᚦᚨᛞ");
    }
}

