#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a>
{
    // Symbols
    COLON(u32), SEMICOLON(u32), COMMA(u32), EQUAL(u32), EQUALEQUAL(u32), LEFTSQUARE(u32), 
    RIGHTSQUARE(u32), LEFTBRACE(u32), RIGHTBRACE(u32), LEFTPARENTHESIS(u32), RIGHTPARENTHESIS(u32),
    AND(u32), OR(u32), LESSTHAN(u32), LESSOREQUAL(u32), GREATERTHAN(u32), GREATEROREQUAL(u32),
    NOT(u32), NOTEQUAL(u32), PLUS(u32), PLUSPLUS(u32), MINUS(u32), MINUSMINUS(u32), TIMES(u32), 
    DIVIDE(u32), DOT(u32),

    // Reserved words
    ARRAY(u32), BOOLEAN(u32), BREAK(u32), CHAR(u32), CONTINUE(u32), DO(u32), FUNCTION(u32),
    IF(u32), ELSE(u32), INTEGER(u32), OF(u32), STRING(u32), STRUCT(u32), TRUE(u32), FALSE(u32),
    TYPE(u32), VAR(u32), WHILE(u32),

    
    // Literals and identifiers
    ID(&'a str), CHARACTER(char), STRINGVAL(&'a str), NUMERAL(&'a str),

    UNKNOWN
}

pub struct TokenStream<'a> {
    it: std::str::Chars<'a>,
    line: u32,
}

impl<'a> TokenStream<'a>{
    pub fn new(it: std::str::Chars<'a>) -> Self {
        TokenStream{
            it, 
            line: 1,
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        while self.it.clone().next().map_or(false, |ch| ch.is_whitespace()){
            if let Some('\n') = self.it.next(){
                self.line += 1;
            }
        }

        match self.it.clone().next() {
            Some(ch) => {
                match ch { 
                    ':' => {
                        self.it.next();
                        Some(Token::COLON(self.line))
                    },
                    ';' => {
                        self.it.next();
                        Some(Token::SEMICOLON(self.line))
                    },
                    ',' => {
                        self.it.next();
                        Some(Token::COMMA(self.line))
                    },
                    '=' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::EQUALEQUAL(self.line))
                        }
                        else {
                            Some(Token::EQUAL(self.line))
                        }
                    },
                    '[' => {
                        self.it.next();
                        Some(Token::LEFTSQUARE(self.line))
                    },
                    ']' => {
                        self.it.next();
                        Some(Token::RIGHTSQUARE(self.line))
                    },
                    '{' => {
                        self.it.next();
                        Some(Token::LEFTBRACE(self.line))
                    },
                    '}' => {
                        self.it.next();
                        Some(Token::RIGHTBRACE(self.line))
                    },
                    '(' => {
                        self.it.next();
                        Some(Token::LEFTPARENTHESIS(self.line))
                    },
                    ')' => {
                        self.it.next();
                        Some(Token::RIGHTPARENTHESIS(self.line))
                    },
                    '&' => {
                        self.it.next();
                        if let Some('&') = self.it.next() {
                            Some(Token::AND(self.line))
                        }
                        else {
                            Some(Token::UNKNOWN)
                        }
                    }
                    '|' => {
                        self.it.next();
                        if let Some('|') = self.it.next() {
                            Some(Token::OR(self.line))
                        }
                        else {
                            Some(Token::UNKNOWN)
                        }
                    }
                    '<' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::LESSOREQUAL(self.line))
                        }
                        else {
                            Some(Token::LESSTHAN(self.line))
                        }
                    },
                    '>' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::GREATEROREQUAL(self.line))
                        }
                        else {
                            Some(Token::GREATERTHAN(self.line))
                        }
                    },
                    '!' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::NOTEQUAL(self.line))
                        }
                        else {
                            Some(Token::NOT(self.line))
                        }
                    },
                    '+' => {
                        self.it.next();
                        if let Some('+') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::PLUSPLUS(self.line))
                        } 
                        else {
                            Some(Token::PLUS(self.line))
                        }
                    },
                    '-' => {
                        self.it.next();
                        if let Some('-') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::MINUSMINUS(self.line))
                        }
                        else {
                            Some(Token::MINUS(self.line))
                        }
                    },
                    '*' => {
                        self.it.next();
                        Some(Token::TIMES(self.line))
                    },
                    '/' => {
                        self.it.next();
                        Some(Token::DIVIDE(self.line))
                    }
                    '.' => {
                        self.it.next();
                        Some(Token::DOT(self.line))
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let str = self.it.as_str();
                        while self.it.clone().next().map_or(false, |ch| ch.is_alphanumeric()) {
                            self.it.next();
                        }

                        // check if the identifier is a reserved word
                        let identifier: &str = &str[..str.len() - self.it.as_str().len()];
                        match identifier {
                            "array" => Some(Token::ARRAY(self.line)),
                            "boolean" => Some(Token::BOOLEAN(self.line)),
                            "break" => Some(Token::BREAK(self.line)),
                            "char" => Some(Token::CHAR(self.line)),
                            "continue" => Some(Token::CONTINUE(self.line)),
                            "do" => Some(Token::DO(self.line)),
                            "function" => Some(Token::FUNCTION(self.line)),
                            "if" => Some(Token::IF(self.line)),
                            "else" => Some(Token::ELSE(self.line)),
                            "integer" => Some(Token::INTEGER(self.line)),
                            "of" => Some(Token::OF(self.line)),
                            "string" => Some(Token::STRING(self.line)),
                            "struct" => Some(Token::STRUCT(self.line)),
                            "true" => Some(Token::TRUE(self.line)),
                            "false" => Some(Token::FALSE(self.line)),
                            "type" => Some(Token::TYPE(self.line)),
                            "var" => Some(Token::VAR(self.line)),
                            "while" => Some(Token::WHILE(self.line)),
                            _ => Some(Token::ID(identifier)),
                        }
                    },
                    '\'' => {
                        self.it.next();
                        let temp = self.it.next();
                        if let Some('\'') = self.it.next() {
                            Some(Token::CHARACTER(temp.expect("Invalid character\n")))
                        } else {
                            Some(Token::UNKNOWN)
                        }
                    },
                    '"' => {
                       self.it.next();
                       let mut ret_token: Token = Token::UNKNOWN;
                       let str = self.it.as_str();
                       while let Some(ch) = self.it.next() {
                            match ch {
                                '\n' => {
                                    self.line+=1;
                                },
                                '"' => {
                                    ret_token = Token::STRINGVAL(&str[..str.len() - self.it.as_str().len()-1]);
                                }
                                _ => ()
                            }
                       }

                       Some(ret_token)
                    },
                    '0'..='9' => {
                        let str = self.it.as_str();
                        while self.it.clone().next().map_or(false, |ch| ch.is_numeric()) {
                            self.it.next();
                        }

                        // In case it is a floating point number
                        if let Some('.') = self.it.clone().next() {
                            self.it.next();
                            while self.it.clone().next().map_or(false, |ch| ch.is_numeric()) {
                                self.it.next();
                            }   
                        }

                        Some(Token::NUMERAL(&str[..str.len() - self.it.as_str().len()]))
                    },
                    _ => Some(Token::UNKNOWN)
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colon(){
        let program = String::from(":");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::COLON(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_semicolon(){
        let program = String::from(";");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::SEMICOLON(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_comma(){
        let program = String::from(",");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::COMMA(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_equal(){
        let program = String::from("=");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::EQUAL(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_equalequal(){
        let program = String::from("==");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::EQUALEQUAL(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_leftsquare(){
        let program = String::from("[");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::LEFTSQUARE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_rightsquare(){
        let program = String::from("]");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::RIGHTSQUARE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_leftbrace(){
        let program = String::from("{");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::LEFTBRACE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_rightbrace(){
        let program = String::from("}");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::RIGHTBRACE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_leftparenthesis(){
        let program = String::from("(");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_rightparenthesis(){
        let program = String::from(")");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_and(){
        let program = String::from("&&");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::AND(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_or(){
        let program = String::from("||");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::OR(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_lessthan(){
        let program = String::from("<");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::LESSTHAN(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_lessorequal(){
        let program = String::from("<=");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::LESSOREQUAL(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_greaterthan(){
        let program = String::from(">");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::GREATERTHAN(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_greaterorequal(){
        let program = String::from(">=");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::GREATEROREQUAL(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_not(){
        let program = String::from("!");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::NOT(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_notequal(){
        let program = String::from("!=");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::NOTEQUAL(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_plus(){
        let program = String::from("+");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::PLUS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_plusplus(){
        let program = String::from("++");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::PLUSPLUS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_minus(){
        let program = String::from("-");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::MINUS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_minusminus(){
        let program = String::from("--");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::MINUSMINUS(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_times(){
        let program = String::from("*");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::TIMES(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_divide(){
        let program = String::from("/");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::DIVIDE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_dot(){
        let program = String::from(".");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::DOT(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_array(){
        let program = String::from("array");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::ARRAY(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_boolean(){
        let program = String::from("boolean");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::BOOLEAN(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_break(){
        let program = String::from("break");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::BREAK(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_char(){
        let program = String::from("char");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::CHAR(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_continue(){
        let program = String::from("continue");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::CONTINUE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_do(){
        let program = String::from("do");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::DO(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_function(){
        let program = String::from("function");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::FUNCTION(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_if(){
        let program = String::from("if");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::IF(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_else(){
        let program = String::from("else");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::ELSE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_integer(){
        let program = String::from("integer");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::INTEGER(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_of(){
        let program = String::from("of");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::OF(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_string(){
        let program = String::from("string");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::STRING(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_struct(){
        let program = String::from("struct");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::STRUCT(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_true(){
        let program = String::from("true");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::TRUE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_false(){
        let program = String::from("false");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::FALSE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_type(){
        let program = String::from("type");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::TYPE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_var(){
        let program = String::from("var");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::VAR(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_while(){
        let program = String::from("while");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::WHILE(1))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_id(){
        let program = String::from("identifier");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::ID("identifier"))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_character(){
        let program = String::from("'a'");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::CHARACTER('a'))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_stringval(){
        let program = String::from(r#""Hello   World""#);
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::STRINGVAL("Hello World"))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_numeral(){
        let program = String::from("123.456");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::NUMERAL("123.456"))); 
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_unknown_from_and(){
        let program = String::from("&a");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_unknown_from_or(){
        let program = String::from("|a");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_unknown_from_end(){
        let program = String::from(r#""sdfdsfdsf"#);
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
        assert_eq!(t_stream.next(), None);
    }

    #[test]
    fn test_unknown_from_character(){
        let program = String::from("'a");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
        assert_eq!(t_stream.next(), None);
    }


    #[test]
    fn test_mix_and_next_line() {
        let program = String::from("struct while() array if else && ||\ncontinue \ndo");
        let mut t_stream = TokenStream::new(program.chars());
        assert_eq!(t_stream.next(), Some(Token::STRUCT(1)));
        assert_eq!(t_stream.next(), Some(Token::WHILE(1)));
        assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS(1)));
        assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS(1)));
        assert_eq!(t_stream.next(), Some(Token::ARRAY(1)));
        assert_eq!(t_stream.next(), Some(Token::IF(1)));
        assert_eq!(t_stream.next(), Some(Token::ELSE(1)));
        assert_eq!(t_stream.next(), Some(Token::AND(1)));
        assert_eq!(t_stream.next(), Some(Token::OR(1)));
        assert_eq!(t_stream.next(), Some(Token::CONTINUE(2)));
        assert_eq!(t_stream.next(), Some(Token::DO(3)));
        assert_eq!(t_stream.next(), None);
    }
}
