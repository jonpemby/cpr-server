use super::token::Token;

pub struct Parser {
    content: String,
    pointer: i32,
}

impl Parser {
    pub fn new(content: String) -> Parser {
        Parser { content, pointer: 0 }
    }

    pub fn read_until_token(&mut self) -> Token {
        let mut stack: Vec<Token> = Vec::new();
        let mut buf = String::new();

        for c in self.content.chars().skip(self.pointer as usize) {
            self.pointer += 1;

            match c {
                '\n' => {
                    let tok = stack.pop();

                    if tok.is_none() {
                        return Token::StringValue(buf);
                    }

                    match tok.unwrap() {
                        Token::ArrayOpen => return Token::ArrayValue(buf),
                        _ => return Token::Break,
                    }
                },
                ':' => return Token::Key(buf),
                '-' => stack.push(Token::ArrayOpen),
                ' ' => {
                    let last = stack.pop();

                    if last.is_none() {
                        continue;
                    }

                    match last.unwrap() {
                        Token::ArrayOpen => stack.push(Token::ArrayWhitespace),
                        Token::ArrayWhitespace => {
                            stack.push(Token::ArrayWhitespace);
                            buf.push(c);
                        },
                        _ => buf.push(c),
                    }
                },

                _ => buf.push(c),
            }
        }

        if buf.is_empty() {
            return Token::None
        }

        if stack.is_empty() {
            return Token::StringValue(buf)
        }

        match stack.pop().unwrap() {
            Token::ArrayWhitespace => return Token::ArrayValue(buf),
            Token::ArrayOpen => return Token::ArrayValue(buf),
            _ => panic!("Improperly formatted file"),
        }
    }
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.read_until_token();

        match tok {
            Token::None => return Option::None,
            _ => return Option::Some(tok),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::config::token::Token;

    #[test]
    fn test_it_reads_none() {
        let mut parser = Parser::new("".to_string());
        let res = parser.next();

        assert_eq!(true, res.is_none());
    }

    #[test]
    fn test_it_reads_keys() {
        let mut parser = Parser::new("key: value".to_string());
        let res = parser.next();

        assert_eq!(true, res.is_some());
        assert_eq!("key", match res.unwrap() {
            Token::Key(s) => s,
            _ => "Not a key value".to_string(),
        });
    }

    #[test]
    fn test_it_reads_string_values() {
        let mut parser = Parser::new("key: value".to_string());
        parser.next().unwrap();

        let res = parser.next();

        assert_eq!(true, res.is_some());
        assert_eq!("value", match res.unwrap() {
            Token::StringValue(s) => s,
            _ => "Not a string value".to_string(),
        });
    }

    #[test]
    fn test_it_reads_array_values() {
        let mut parser = Parser::new("- array value".to_string());

        let res = parser.next();

        assert_eq!(true, res.is_some());
        assert_eq!("array value", match res.unwrap() {
            Token::ArrayValue(s) => s,
            _ => "Not an array value".to_string(),
        });
    }
}