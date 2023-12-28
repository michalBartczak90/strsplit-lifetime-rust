// #![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack, '_> { // '_ tells that lifetime is not needed for this block
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c)).next().expect("Error")
}


#[cfg(test)]
mod tests {
    use super::*;

    fn until_char_test() {
        assert_eq!(until_char("hello world", 'l'), "he")
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }
}