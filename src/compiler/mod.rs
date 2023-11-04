mod scanner;
mod token;

pub fn compile(source: &str) {
    let mut scanner = scanner::Scanner::init(source);

    loop {
        let token = scanner.scan_token();
        println!("{:?}", token);

        if token.token_type == token::TokenType::Eof {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{compile, scanner::Scanner, token::TokenType};

    #[test]
    fn scan_string_token() {
        let mut scanner = Scanner::init("\"slice\"");
        let token = scanner.scan_token();

        match &token.token_type {
            TokenType::String(x) => println!("{}", x),
            _ => println!("Unknown"),
        }

        assert_eq!(token.token_type, TokenType::String(String::from("slice\"")));
    }

    #[test]
    fn scan_unterminated_string_token() {
        let mut scanner = Scanner::init("\"slice");
        let token = scanner.scan_token();

        assert_eq!(token.token_type, TokenType::Error("Unterminated string"));
    }

    #[test]
    fn scan_with_whitespace() {
        let mut scanner = Scanner::init("  >=  ");
        let token = scanner.scan_token();

        assert_eq!(token.token_type, TokenType::GreaterEqual);
    }

    #[test]
    fn scan_one_line() {
        compile("+ == /");
    }

    #[test]
    fn scan_two_lines() {
        compile("+ + \n-");
    }

    #[test]
    fn scan_with_comment() {
        compile("+#hello\n-");
    }

    #[test]
    fn scan_number() {
        compile("+90rust");
    }

    #[test]
    fn scan_decimal_number() {
        compile("+93.5 = red fm");
    }

    #[test]
    fn scan_number_plus_number() {
        compile("2 + 3\n1 + 2 = 3");
    }

    #[test]
    fn statement() {
        compile("fun main() {}");
    }
}
