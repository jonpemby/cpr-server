struct Line {
    contents: String,
    ops: Vec<Token>,
}

impl Line {
    pub fn new(contents: String) -> Line {
        Line { contents, ops: Vec::new() }
    }

    pub fn parse(&mut self, string: String) -> Token {
        let mut buf = String::new();

        self.ops.push(Token::Start);

        for c in string.chars() {
            match c {
                '\n' => break,
                ':' => {
                    let tok = self.get_last_op();

                    match tok.unwrap() {
                        Token::Start => return Token::Key(buf),
                        _ => panic!("Failed"),
                    }
                },
                '-' => self.ops.push(Token::ArrayOpen),
                ' ' => {
                    let tok = self.get_last_op();

                    match tok.unwrap() {
                        Token::ArrayOpen => continue,
                        _ => buf.push(c),
                    }
                },


                _ => buf.push(c),
            }
        }

        let tok = self.get_last_op();

        match tok.unwrap() {
            Token::ArrayOpen => return Token::ArrayValue(buf),
            Token::Start => return Token::StringValue(buf),
            _ => Token::Empty,
        }
    }

    fn get_last_op(&self) -> Option<&Token> {
        let last_index = self.ops.len() - 1;

        if last_index < 0 {
            return Option::None
        }

        Option::Some(&self.ops[last_index])
    }
}

impl Iterator for Line {
    fn next(&self) -> Option<Token> {

    }
}

enum Token {
    Start,
    Key(String),
    ArrayValue(String),
    StringValue(String),
    ArrayOpen,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::{Line, Token};

    #[test]
    fn test_parses_keys() {
        let mut line = Line::new(String::from("key:"));
        let tok= line.parse();

        assert_eq!("key", match tok {
            Token::Key(key) => key,
            _ => String::from("Not a key value"),
        });
    }

    #[test]
    fn test_parses_arrays() {
        let mut line = Line::new(String::from("- value"));
        let tok= line.parse();

        assert_eq!("value", match tok {
            Token::ArrayValue(val) => val,
            _ => String::from("Not an array value"),
        });
    }

    #[test]
    fn test_parses_strings() {
        let mut line = Line::new(String::from("string value"));
        let tok = line.parse();

        assert_eq!("string value", match tok {
            Token::StringValue(val) => val,
            _ => String::from("Not a string value"),
        });
    }
}