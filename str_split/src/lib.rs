#![warn(missing_debug_implementations)]

/// This is a lib for playing with lifetimes

#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    fn new(s: &'a str, d: &'b str) -> Self {
        Self {
            remainder: Some(s),
            delimiter: d,
        }
    }
}

// here 'delimter is.. not an issue because it's not needed to be returned out of scope
impl<'a> Iterator for StrSplit<'a, '_> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut rmd) = self.remainder {
            if let Some(next_delim) = rmd.find(self.delimiter) {
                let until_delim = &rmd[..next_delim];
                // rmd: &mut &'a str
                *rmd = &rmd[(next_delim + self.delimiter.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[test]
fn new_str_split_works() {
    let str_to_split = StrSplit::new("hello world", "o");
    assert_eq!(str_to_split.remainder, Some("hello world"));
}

#[test]
fn collect_split_works() {
    let str_to_split: Vec<_> = StrSplit::new("hello world", "o").collect();
    let split = ["hell", " w", "rld"];
    assert_eq!(str_to_split, split);
}
