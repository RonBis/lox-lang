use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i16,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        // self.start represents starting of each lexeme/token
        self.start = self.current;

        let mut iterator = self.source[self.start..].as_bytes().iter();

        let token_type = loop {
            let c = iterator.next();
            if c.is_none() {
                break TokenType::Eof;
            }

            // println!(
            //     "start: {}, current: {}, ch: {}",
            //     self.start,
            //     self.current,
            //     c.is_some()
            // );

            self.current += 1;

            break match c.unwrap() {
                b'(' => TokenType::LeftParen,
                b')' => TokenType::RightParen,
                b'{' => TokenType::LeftBrace,
                b'}' => TokenType::RightParen,
                b';' => TokenType::Semicolon,
                b',' => TokenType::Comma,
                b'-' => TokenType::Minus,
                b'+' => TokenType::Plus,
                b'/' => TokenType::Slash,
                b'*' => TokenType::Star,

                // two-character tokens: !=, ==, <=, >=
                b'!' => {
                    if iterator.next().is_some_and(|&x| x == b'=') {
                        self.current += 1;
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                // b'=' => match iterator.next() {
                //     Some(&x) => {
                //         if x == b'=' {
                //             self.current += 1;
                //             TokenType::EqualEqual
                //         } else {
                //             TokenType::Equal
                //         }
                //     }
                //     None => TokenType::Equal,
                // },
                b'=' => {
                    if iterator.next().is_some_and(|&x| x == b'=') {
                        self.current += 1;
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                b'<' => {
                    if iterator.next().is_some_and(|&x| x == b'=') {
                        self.current += 1;
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                b'>' => {
                    if iterator.next().is_some_and(|&x| x == b'=') {
                        self.current += 1;
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }

                // identifiers
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                    while let Some(x) = iterator.next() {
                        match x {
                            // identifiers can include digits (not at start)
                            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9' => {
                                self.current += 1;
                                continue;
                            }
                            _ => break,
                        }
                    }
                    TokenType::Identifier
                }

                // numbers
                b'0'..=b'9' => {
                    while let Some(x) = iterator.next() {
                        match x {
                            b'0'..=b'9' | b'.' => {
                                self.current += 1;
                                continue;
                            }
                            _ => break,
                        }
                    }
                    TokenType::Number
                }

                // string literals
                b'"' => {
                    let mut tk = None;
                    let mut str = String::new();

                    while let Some(x) = iterator.next() {
                        self.current += 1;
                        str.push(*x as char);

                        if *x == b'"' {
                            tk = Some(TokenType::String);
                            break;
                        }
                        if *x == b'\n' {
                            self.line += 1;
                        }
                    }

                    match tk {
                        Some(_) => TokenType::String(str),
                        None => TokenType::Error("Unterminated string"),
                    }
                }

                // skip whitespace
                b' ' | b'\r' | b'\t' => {
                    self.start += 1;
                    continue;
                }
                b'\n' => {
                    self.start += 1;
                    self.line += 1;
                    continue;
                }

                // comments
                b'#' => {
                    self.start += 1;

                    while let Some(x) = iterator.next() {
                        self.start += 1;
                        self.current += 1;

                        if *x == b'\n' {
                            self.line += 1;
                            break;
                        }
                    }
                    continue;
                }

                _ => TokenType::Error("Unknown token"),
            };
        };

        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }
}
