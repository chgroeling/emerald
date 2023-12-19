#[derive(Debug)]
pub struct Utf8Iterator<'a> {
    slice: &'a str,
    slice_len: usize,
    byte_pos: usize,

    next_char: Option<(usize, char)>, // Cache for the next character
}

impl<'a> Utf8Iterator<'a> {
    pub fn new(slice: &'a str) -> Utf8Iterator<'a> {
        let mut iterator = Utf8Iterator {
            slice,
            byte_pos: 0,
            slice_len: slice.len(),
            next_char: None,
        };
        iterator.next_char = iterator.peek_next();
        iterator
    }

    fn peek_next(&self) -> Option<(usize, char)> {
        if self.byte_pos < self.slice_len {
            let first_byte = self.slice.as_bytes()[self.byte_pos];
            let char_len = match first_byte {
                0x00..=0x7F => return Some((self.byte_pos, first_byte as char)),
                0xC0..=0xDF => 2,
                0xE0..=0xEF => 3,
                0xF0..=0xF7 => 4,
                _ => panic!("Invalid UTF-8 start byte"),
            };

            let char_bytes = &self.slice.as_bytes()[self.byte_pos..self.byte_pos + char_len];
            let char = std::str::from_utf8(char_bytes)
                .unwrap()
                .chars()
                .next()
                .unwrap();

            Some((self.byte_pos, char))
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&(usize, char)> {
        self.next_char.as_ref()
    }
}

impl<'a> Iterator for Utf8Iterator<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((pos, ch)) = self.next_char.take() {
            self.byte_pos = pos + ch.len_utf8();
            self.next_char = self.peek_next();
            Some((pos, ch))
        } else {
            None
        }
    }
}
