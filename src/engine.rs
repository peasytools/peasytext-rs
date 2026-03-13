/// Convert text to UPPERCASE.
pub fn uppercase(text: &str) -> String { text.to_uppercase() }

/// Convert text to lowercase.
pub fn lowercase(text: &str) -> String { text.to_lowercase() }

/// Convert text to Title Case.
pub fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + &c.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Convert text to camelCase.
pub fn camel_case(text: &str) -> String {
    let words = split_words(text);
    words.iter().enumerate().map(|(i, w)| {
        if i == 0 { w.to_lowercase() } else { title_word(w) }
    }).collect()
}

/// Convert text to snake_case.
pub fn snake_case(text: &str) -> String {
    split_words(text).iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("_")
}

/// Convert text to kebab-case.
pub fn kebab_case(text: &str) -> String {
    split_words(text).iter().map(|w| w.to_lowercase()).collect::<Vec<_>>().join("-")
}

/// Convert text to PascalCase.
pub fn pascal_case(text: &str) -> String {
    split_words(text).iter().map(|w| title_word(w)).collect()
}

/// Convert text to CONSTANT_CASE.
pub fn constant_case(text: &str) -> String {
    split_words(text).iter().map(|w| w.to_uppercase()).collect::<Vec<_>>().join("_")
}

/// Count words in text.
pub fn word_count(text: &str) -> usize { text.split_whitespace().count() }

/// Count characters in text.
pub fn char_count(text: &str) -> usize { text.chars().count() }

/// Count lines in text.
pub fn line_count(text: &str) -> usize { text.lines().count() }

/// Generate a URL-safe slug.
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Reverse text.
pub fn reverse(text: &str) -> String { text.chars().rev().collect() }

/// Truncate text to a maximum length.
pub fn truncate(text: &str, length: usize, suffix: &str) -> String {
    if text.len() <= length { return text.to_string(); }
    let end = length.saturating_sub(suffix.len());
    format!("{}{}", &text[..end], suffix)
}

/// Base64 encode.
pub fn base64_encode(text: &str) -> String {
    use std::io::Write;
    let mut buf = Vec::new();
    {
        let mut enc = Base64Encoder::new(&mut buf);
        enc.write_all(text.as_bytes()).unwrap();
        enc.finish();
    }
    String::from_utf8(buf).unwrap()
}

/// Base64 decode.
pub fn base64_decode(text: &str) -> Result<String, std::string::FromUtf8Error> {
    let bytes = base64_decode_bytes(text);
    String::from_utf8(bytes)
}

fn split_words(text: &str) -> Vec<String> {
    let mut result = String::new();
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        if i > 0 && chars[i].is_uppercase() && chars[i-1].is_lowercase() {
            result.push(' ');
        }
        if chars[i] == '-' || chars[i] == '_' || chars[i] == '.' {
            result.push(' ');
        } else {
            result.push(chars[i]);
        }
    }
    result.split_whitespace().map(String::from).collect()
}

fn title_word(w: &str) -> String {
    let mut c = w.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().to_string() + &c.as_str().to_lowercase(),
    }
}

// Minimal Base64 encoder (no external deps)
const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

struct Base64Encoder<'a> { out: &'a mut Vec<u8>, buf: [u8; 3], len: usize }
impl<'a> Base64Encoder<'a> {
    fn new(out: &'a mut Vec<u8>) -> Self { Self { out, buf: [0; 3], len: 0 } }
    fn finish(&mut self) {
        if self.len > 0 {
            for i in self.len..3 { self.buf[i] = 0; }
            let b = self.buf;
            self.out.push(B64[(b[0] >> 2) as usize]);
            self.out.push(B64[((b[0] & 3) << 4 | b[1] >> 4) as usize]);
            if self.len > 1 { self.out.push(B64[((b[1] & 0xf) << 2 | b[2] >> 6) as usize]); } else { self.out.push(b'='); }
            self.out.push(b'=');
        }
    }
}
impl std::io::Write for Base64Encoder<'_> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        for &byte in data {
            self.buf[self.len] = byte; self.len += 1;
            if self.len == 3 {
                let b = self.buf;
                self.out.push(B64[(b[0] >> 2) as usize]);
                self.out.push(B64[((b[0] & 3) << 4 | b[1] >> 4) as usize]);
                self.out.push(B64[((b[1] & 0xf) << 2 | b[2] >> 6) as usize]);
                self.out.push(B64[(b[2] & 0x3f) as usize]);
                self.len = 0;
            }
        }
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

const B64_DEC: [u8; 128] = {
    let mut t = [255u8; 128];
    let mut i = 0u8;
    while i < 64 { t[B64[i as usize] as usize] = i; i += 1; }
    t
};

fn base64_decode_bytes(text: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let bytes: Vec<u8> = text.bytes().filter(|&b| b != b'\n' && b != b'\r').collect();
    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 { break; }
        let a = B64_DEC[chunk[0] as usize];
        let b = B64_DEC[chunk[1] as usize];
        out.push(a << 2 | b >> 4);
        if chunk.len() > 2 && chunk[2] != b'=' {
            let c = B64_DEC[chunk[2] as usize];
            out.push((b & 0xf) << 4 | c >> 2);
            if chunk.len() > 3 && chunk[3] != b'=' {
                let d = B64_DEC[chunk[3] as usize];
                out.push((c & 3) << 6 | d);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_conversions() {
        assert_eq!(uppercase("hello"), "HELLO");
        assert_eq!(lowercase("HELLO"), "hello");
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(camel_case("hello world"), "helloWorld");
        assert_eq!(snake_case("helloWorld"), "hello_world");
        assert_eq!(kebab_case("hello_world"), "hello-world");
        assert_eq!(pascal_case("hello world"), "HelloWorld");
        assert_eq!(constant_case("hello world"), "HELLO_WORLD");
    }

    #[test]
    fn test_counts() {
        assert_eq!(word_count("hello world foo"), 3);
        assert_eq!(char_count("hello"), 5);
        assert_eq!(line_count("a\nb\nc"), 3);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!"), "hello-world");
    }

    #[test]
    fn test_base64() {
        let encoded = base64_encode("hello");
        assert_eq!(encoded, "aGVsbG8=");
        assert_eq!(base64_decode(&encoded).unwrap(), "hello");
    }
}
