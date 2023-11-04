use super::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i16,
}

impl<'a> Scanner<'a> {
    /// Initialises the scanner with given `source` string
    pub fn init(source: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Scans individual tokens from given source
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
                b'}' => TokenType::RightBrace,
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

                    let identifier = Self::make_identifier(&self.source[self.start..self.current]);
                    identifier
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

    /// Makes keyword or identifier from given token slice
    fn make_identifier(slice: &str) -> TokenType {
        // check if length is equal to expected length and matched with `match_with`
        let check_keyword = |start: usize,
                             expected_length: usize,
                             match_with: &'static str,
                             expected_keyword: TokenType| {
            let end = start + expected_length;

            if slice.len() == start + expected_length && slice[start..end] == *match_with {
                expected_keyword
            } else {
                TokenType::Identifier
            }
        };

        let slice = slice.as_bytes();

        match slice[0] {
            // keywords: and, class, else, if, nil, or, print, return, super, var, while
            b'a' => check_keyword(1, 2, "nd", TokenType::And),
            b'c' => check_keyword(1, 4, "lass", TokenType::Class),
            b'e' => check_keyword(1, 3, "lse", TokenType::Else),
            b'i' => check_keyword(1, 1, "f", TokenType::If),
            b'n' => check_keyword(1, 2, "il", TokenType::Nil),
            b'o' => check_keyword(1, 1, "r", TokenType::Or),
            b'p' => check_keyword(1, 4, "rint", TokenType::Print),
            b'r' => check_keyword(1, 5, "eturn", TokenType::Return),
            b's' => check_keyword(1, 4, "uper", TokenType::Super),
            b'v' => check_keyword(1, 2, "ar", TokenType::Var),
            b'w' => check_keyword(1, 4, "hile", TokenType::While),

            // branched keywords: false, for, fun
            b'f' => {
                if slice.len() > 1 {
                    match slice[1] {
                        b'a' => check_keyword(2, 3, "lse", TokenType::False),
                        b'o' => check_keyword(2, 1, "r", TokenType::For),
                        b'u' => check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            // branched keywords: this, true
            b't' => {
                if slice.len() > 1 {
                    match slice[1] {
                        b'h' => check_keyword(2, 2, "is", TokenType::This),
                        b'r' => check_keyword(2, 3, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }

            _ => TokenType::Identifier,
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::compiler::{scanner::Scanner, token::TokenType};

    #[test]
    fn scan_print() {
        let mut scanner = Scanner::init("print 1+2;");
        let token = scanner.scan_token().token_type;

        assert_eq!(token, TokenType::Print);
    }
}
