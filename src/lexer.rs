#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Equal,
    Print,
    Let,
    Identifier(String),
    Number(f64),
    String(String),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current = self.input[self.position];

            match current {
                '=' => {
                    tokens.push(Token::Equal);
                    self.position += 1;
                }

                ' ' | '\t' | '\r' | '\n' => {
                    self.position += 1;
                }

                '(' => {
                    tokens.push(Token::LeftParen);
                    self.position += 1;
                }

                ')' => {
                    tokens.push(Token::RightParen);
                    self.position += 1;
                }

                '+' => {
                    tokens.push(Token::Plus);
                    self.position += 1;
                }

                '-' => {
                    tokens.push(Token::Minus);
                    self.position += 1;
                }

                '*' => {
                    tokens.push(Token::Star);
                    self.position += 1;
                }

                '/' => {
                    tokens.push(Token::Slash);
                    self.position += 1;
                }

                '"' => {
                    tokens.push(Token::String(self.read_string()?));
                }

                c if c.is_ascii_digit() => {
                    tokens.push(Token::Number(self.read_number()?));
                }

                c if c.is_ascii_alphabetic() || c == '_' => {
                    let word = self.read_identifier();

                    match word.as_str() {
                        "print" => tokens.push(Token::Print),
                        "let" => tokens.push(Token::Let),
                        _ => tokens.push(Token::Identifier(word)),
                    }
                }

                _ => {
                    return Err(format!("Unexpected character: '{}'", current));
                }
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<f64, String> {
        let start = self.position;
        let mut dot_count = 0;

        while self.position < self.input.len() {
            let current = self.input[self.position];

            if current.is_ascii_digit() {
                self.position += 1;
            } else if current == '.' {
                dot_count += 1;

                if dot_count > 1 {
                    return Err("Invalid number".to_string());
                }

                self.position += 1;
            } else {
                break;
            }
        }

        let value: String = self.input[start..self.position].iter().collect();

        value
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: '{}'", value))
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.position += 1;

        let mut result = String::new();

        while self.position < self.input.len() {
            let current = self.input[self.position];

            match current {
                '"' => {
                    self.position += 1;
                    return Ok(result);
                }

                '\\' => {
                    self.position += 1;

                    if self.position >= self.input.len() {
                        return Err("Unterminated string escape".to_string());
                    }

                    let escaped = self.input[self.position];

                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => {
                            return Err(format!("Unknown escape sequence: \\{}", escaped));
                        }
                    }

                    self.position += 1;
                }

                _ => {
                    result.push(current);
                    self.position += 1;
                }
            }
        }

        Err("Unterminated string".to_string())
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while self.position < self.input.len() {
            let current = self.input[self.position];

            if current.is_ascii_alphanumeric() || current == '_' {
                self.position += 1;
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_arithmetic() {
        let mut lexer = Lexer::new("10 + 20 * 2");

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Number(10.0),
                Token::Plus,
                Token::Number(20.0),
                Token::Star,
                Token::Number(2.0),
                Token::Eof,
            ]
        );
    }

    #[test]
    fn tokenizes_print_string() {
        let mut lexer = Lexer::new(r#"print("Hello, Nexus!")"#);

        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Print,
                Token::LeftParen,
                Token::String("Hello, Nexus!".to_string()),
                Token::RightParen,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn rejects_unknown_identifier() {
        let mut lexer = Lexer::new("hello");

        assert!(lexer.tokenize().is_err());
    }
}
