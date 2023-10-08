#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a>
{
    // Symbols
    COLON, SEMICOLON, COMMA, EQUAL, EQUALEQUAL, LEFTSQUARE, 
    RIGHTSQUARE, LEFTBRACE, RIGHTBRACE, LEFTPARENTHESIS, RIGHTPARENTHESIS,
    AND, OR, LESSTHAN, LESSOREQUAL, GREATERTHAN, GREATEROREQUAL,
    NOT, NOTEQUAL, PLUS, PLUSPLUS, MINUS, MINUSMINUS, TIMES, 
    DIVIDE, DOT,

    // Reserved words
    ARRAY, BOOLEAN, BREAK, CHAR, CONTINUE, DO, FUNCTION,
    IF, ELSE, INTEGER, OF, RETURN, STRING, STRUCT, TRUE, FALSE,
    TYPE, VAR, WHILE,

    // Mark the end of program
    DOLLAR,
    
    // Literals and identifiers
    ID(&'a str), CHARACTER(char), STRINGVAL(&'a str), NUMERAL(&'a str),

    UNKNOWN(&'a str)
}

impl<'a> From<Token<'a>> for usize {
    fn from(t: Token<'a>) -> Self {
        match t {
            Token::ARRAY => 35,
            Token::BOOLEAN => 36,
            Token::BREAK => 37,
            Token::CHAR => 38,
            Token::CONTINUE => 39,
            Token::DO => 40,
            Token::ELSE => 41,
            Token::FALSE => 42,
            Token::FUNCTION => 43,
            Token::IF => 44,
            Token::INTEGER => 45,
            Token::OF => 46,
            Token::RETURN => 47,
            Token::STRING => 48,
            Token::STRUCT => 49,
            Token::TRUE => 50,
            Token::TYPE => 51,
            Token::VAR => 52,
            Token::WHILE => 53,
            Token::COLON => 54,
            Token::SEMICOLON => 55,
            Token::COMMA => 56,
            Token::EQUAL => 57,
            Token::LEFTSQUARE => 58,
            Token::RIGHTSQUARE => 59,
            Token::LEFTBRACE => 60,
            Token::RIGHTBRACE => 61,
            Token::LEFTPARENTHESIS => 62,
            Token::RIGHTPARENTHESIS => 63,
            Token::AND => 64,
            Token::OR => 65,
            Token::LESSTHAN => 66,
            Token::GREATERTHAN => 67,
            Token::LESSOREQUAL => 68,
            Token::GREATEROREQUAL => 69,
            Token::NOTEQUAL => 70,
            Token::EQUALEQUAL => 71,
            Token::PLUS => 72,
            Token::PLUSPLUS => 73,
            Token::MINUS => 74,
            Token::MINUSMINUS => 75,
            Token::TIMES => 76,
            Token::DIVIDE => 77,
            Token::DOT => 78,
            Token::NOT => 79,
            Token::CHARACTER(_) => 80,
            Token::NUMERAL(_) => 81,
            Token::STRINGVAL(_) => 82,
            Token::ID(_) => 83,
            Token::DOLLAR => 84,
            Token::UNKNOWN(_) => 1000,
        }
    }
}
