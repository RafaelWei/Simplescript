use crate::syntatic_analyzer::terminals::Token;

pub struct TokenStream<'a> {
    it: std::str::Chars<'a>,
    line: u32,
}

impl<'a> TokenStream<'a>{ pub fn new(it: std::str::Chars<'a>) -> Self {
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
                        Some(Token::COLON)
                    },
                    ';' => {
                        self.it.next();
                        Some(Token::SEMICOLON)
                    },
                    ',' => {
                        self.it.next();
                        Some(Token::COMMA)
                    },
                    '=' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::EQUALEQUAL)
                        }
                        else {
                            Some(Token::EQUAL)
                        }
                    },
                    '[' => {
                        self.it.next();
                        Some(Token::LEFTSQUARE)
                    },
                    ']' => {
                        self.it.next();
                        Some(Token::RIGHTSQUARE)
                    },
                    '{' => {
                        self.it.next();
                        Some(Token::LEFTBRACE)
                    },
                    '}' => {
                        self.it.next();
                        Some(Token::RIGHTBRACE)
                    },
                    '(' => {
                        self.it.next();
                        Some(Token::LEFTPARENTHESIS)
                    },
                    ')' => {
                        self.it.next();
                        Some(Token::RIGHTPARENTHESIS)
                    },
                    '&' => {
                        self.it.next();
                        if let Some('&') = self.it.next() {
                            Some(Token::AND)
                        }
                        else {
                            Some(Token::UNKNOWN("&"))
                        }
                    }
                    '|' => {
                        self.it.next();
                        if let Some('|') = self.it.next() {
                            Some(Token::OR)
                        }
                        else {
                            Some(Token::UNKNOWN("|"))
                        }
                    }
                    '<' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::LESSOREQUAL)
                        }
                        else {
                            Some(Token::LESSTHAN)
                        }
                    },
                    '>' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::GREATEROREQUAL)
                        }
                        else {
                            Some(Token::GREATERTHAN)
                        }
                    },
                    '!' => {
                        self.it.next();
                        if let Some('=') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::NOTEQUAL)
                        }
                        else {
                            Some(Token::NOT)
                        }
                    },
                    '+' => {
                        self.it.next();
                        if let Some('+') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::PLUSPLUS)
                        } 
                        else {
                            Some(Token::PLUS)
                        }
                    },
                    '-' => {
                        self.it.next();
                        if let Some('-') = self.it.clone().next() {
                            self.it.next();
                            Some(Token::MINUSMINUS)
                        }
                        else {
                            Some(Token::MINUS)
                        }
                    },
                    '*' => {
                        self.it.next();
                        Some(Token::TIMES)
                    },
                    '/' => {
                        self.it.next();
                        Some(Token::DIVIDE)
                    },
                    '.' => {
                        self.it.next();
                        Some(Token::DOT)
                    },
                    '$' => {
                        self.it.next();
                        Some(Token::DOLLAR)
                    },
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let str = self.it.as_str();
                        while self.it.clone().next().map_or(false, |ch| ch.is_alphanumeric()) {
                            self.it.next();
                        }

                        // check if the identifier is a reserved word
                        let identifier: &str = &str[..str.len() - self.it.as_str().len()];
                        match identifier {
                            "array" => Some(Token::ARRAY),
                            "boolean" => Some(Token::BOOLEAN),
                            "break" => Some(Token::BREAK),
                            "char" => Some(Token::CHAR),
                            "continue" => Some(Token::CONTINUE),
                            "do" => Some(Token::DO),
                            "function" => Some(Token::FUNCTION),
                            "if" => Some(Token::IF),
                            "else" => Some(Token::ELSE),
                            "integer" => Some(Token::INTEGER),
                            "of" => Some(Token::OF),
                            "string" => Some(Token::STRING),
                            "struct" => Some(Token::STRUCT),
                            "true" => Some(Token::TRUE),
                            "false" => Some(Token::FALSE),
                            "type" => Some(Token::TYPE),
                            "var" => Some(Token::VAR),
                            "while" => Some(Token::WHILE),
                            _ => Some(Token::ID(identifier)),
                        }
                    },
                    '\'' => {
                        self.it.next();
                        let temp = self.it.next();
                        if let Some('\'') = self.it.next() {
                            Some(Token::CHARACTER(temp.expect("Invalid character\n")))
                        } else {
                            Some(Token::UNKNOWN("'"))
                        }
                    },
                    '"' => {
                       self.it.next();
                       let mut ret_token: Token = Token::UNKNOWN("AAAAAAAA");
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
                    _ => {
                        let str = self.it.as_str();
                        Some(Token::UNKNOWN(str))
                    }
                }
            }
            None => Some(Token::DOLLAR),
        }
    }
}


