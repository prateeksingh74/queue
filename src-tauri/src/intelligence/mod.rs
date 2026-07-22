use crate::state::ContentType;
use regex::Regex;
use std::sync::OnceLock;

struct Patterns {
    email: Regex,
    url: Regex,
    phone: Regex,
    date: Regex,
    address: Regex,
}

fn patterns() -> &'static Patterns {
    static PATTERNS: OnceLock<Patterns> = OnceLock::new();
    PATTERNS.get_or_init(|| Patterns {
        email: Regex::new(r"(?i)^[a-z0-9._%+\-]+@[a-z0-9.\-]+\.[a-z]{2,}$").unwrap(),
        url: Regex::new(r"^https?://[^\s]+$").unwrap(),
        phone: Regex::new(r"^[\+]?[\d\s\-\.\(\)]{7,20}$").unwrap(),
        date: Regex::new(r"^\d{1,2}[/\-\.]\d{1,2}[/\-\.]\d{2,4}$").unwrap(),
        address: Regex::new(r"(?i)^\d+\s+[\w\s]+(?:st|ave|blvd|dr|rd|ln|ct|way|pl)[\.,]").unwrap(),
    })
}

pub fn classify(content: &str) -> ContentType {
    let s = content.trim();
    if s.is_empty() {
        return ContentType::Unknown;
    }
    let p = patterns();
    if p.email.is_match(s) {
        ContentType::Email
    } else if p.url.is_match(s) {
        ContentType::Url
    } else if p.phone.is_match(s) {
        ContentType::Phone
    } else if p.date.is_match(s) {
        ContentType::Date
    } else if p.address.is_match(s) {
        ContentType::Address
    } else {
        ContentType::Text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_email() {
        assert_eq!(classify("user@example.com"), ContentType::Email);
    }

    #[test]
    fn classifies_url() {
        assert_eq!(classify("https://example.com/path"), ContentType::Url);
    }

    #[test]
    fn classifies_phone() {
        assert_eq!(classify("+1 555-123-4567"), ContentType::Phone);
    }

    #[test]
    fn classifies_plain_text() {
        assert_eq!(classify("John Smith"), ContentType::Text);
    }
}
