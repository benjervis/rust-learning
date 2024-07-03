use std::{iter::Peekable, str::Chars};

use super::token_stream::TokenStream;

pub struct StringStream<'a> {
    data: String,
    chars: Option<Peekable<Chars<'a>>>,
}

impl<'a> Iterator for StringStream<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars_iter().next()
    }
}

impl<'a> From<String> for StringStream<'a> {
    fn from(value: String) -> Self {
        Self {
            data: value,
            chars: None,
        }
    }
}

impl<'a> TokenStream for StringStream<'a> {
    fn take_next_word(&mut self) -> Option<String> {
        let result: String = self
            .chars_iter()
            .take_while(|c| !c.is_whitespace())
            .collect();

        if result.len() == 0 {
            None
        } else {
            Some(result)
        }
    }
}

impl<'a> StringStream<'a> {
    fn chars_iter(&'a mut self) -> &mut Peekable<Chars<'a>> {
        if let Some(chars_iter) = self.chars.as_mut() {
            return chars_iter;
        } else {
            self.chars = Some(self.data.chars().peekable());
            self.chars.as_mut().unwrap()
        }
    }
}
