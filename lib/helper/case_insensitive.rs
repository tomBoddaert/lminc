#[derive(Clone, Copy, Debug)]
/// A case-insensitive char
///
/// ```
/// use lminc::helper::case_insensitive::Char;
///
/// let char_1: Char = 'a'.into();
/// let char_2: Char = 'A'.into();
///
/// assert!(char_1 == char_2);
/// assert_eq!(char_1, char_2);
///
/// let char_3: Char = 'B'.into();
///
/// assert!(char_1 != char_3);
/// assert_ne!(char_1, char_3);
/// ```
pub struct Char(char);

impl From<char> for Char {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl PartialEq for Char {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_ascii_lowercase() == other.0 || self.0.to_ascii_uppercase() == other.0
    }
}

impl Eq for Char {}

impl PartialEq<char> for Char {
    fn eq(&self, other: &char) -> bool {
        &self.0.to_ascii_lowercase() == other || &self.0.to_ascii_uppercase() == other
    }
}

#[derive(Clone, Copy, Debug)]
/// A case-insensitive str
///
/// ```
/// use lminc::helper::case_insensitive::Str;
///
/// let str_1: Str = "HeLlO, WoRlD!".into();
/// let str_2: Str = "hello, world!".into();
///
/// assert!(str_1 == str_2);
/// assert_eq!(str_1, str_2);
///
/// let str_3: Str = "Goodbye".into();
///
/// assert!(str_1 != str_3);
/// assert_ne!(str_1, str_3);
/// ```
pub struct Str<'a>(&'a str);

impl<'a> From<&'a str> for Str<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> PartialEq for Str<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() == other.0.len() {
            self.0
                .chars()
                .map(Char)
                .zip(other.0.chars())
                .all(|(a, b)| a == b)
        } else {
            false
        }
    }
}

impl<'a> Eq for Str<'a> {}

impl<'a> PartialEq<&str> for Str<'a> {
    fn eq(&self, other: &&str) -> bool {
        if self.0.len() == other.len() {
            self.0
                .chars()
                .map(Char::from)
                .zip(other.chars())
                .all(|(a, b)| a == b)
        } else {
            false
        }
    }
}
