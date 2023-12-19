#[derive(Debug)]
pub struct Utf8Iterator<'a> {
    slice: &'a str,
    byte_pos: usize,

    next_char: Option<(usize, char)>, // Cache for the next character
}

impl<'a> Utf8Iterator<'a> {
    pub fn new(slice: &'a str) -> Utf8Iterator<'a> {
        let mut iterator = Utf8Iterator {
            slice,
            byte_pos: 0,
            next_char: None,
        };
        iterator.next_char = iterator.peek_next();
        iterator
    }

    fn char_len(&self, pos: usize) -> usize {
        let first_byte = self.slice.as_bytes()[pos];
        match first_byte {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => panic!("Invalid UTF-8 start byte"),
        }
    }
    fn peek_next(&self) -> Option<(usize, char)> {
        if self.byte_pos < self.slice.len() {
            let char_len = self.char_len(self.byte_pos);
            if char_len == 1 {
                Some((self.byte_pos, self.slice.as_bytes()[self.byte_pos] as char))
            } else {
                let char_bytes = &self.slice.as_bytes()[self.byte_pos..self.byte_pos + char_len];
                let char = std::str::from_utf8(char_bytes)
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();

                Some((self.byte_pos, char))
            }
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
            self.byte_pos = pos + self.char_len(self.byte_pos);
            self.next_char = self.peek_next();
            Some((pos, ch))
        } else {
            None
        }
    }
}
