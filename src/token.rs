#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Space,
    Tab,
    Newline,
}

pub struct Tokens<'a> {
    source: &'a [u8],
    idx: usize,
    line_no: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.as_bytes();

        Self {
            source,
            idx: 0,
            line_no: 1,
        }
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            if self.source.get(self.idx).is_some() {
                let b = self.source[self.idx];
                self.idx += 1;

                match b {
                    b' ' => return Some(Token::Space),
                    b'\t' => return Some(Token::Tab),
                    b'\n' => {
                        self.line_no += 1;
                        return Some(Token::Newline);
                    }
                    _ => continue,
                }
            } else {
                return None;
            }
        }
    }
}
