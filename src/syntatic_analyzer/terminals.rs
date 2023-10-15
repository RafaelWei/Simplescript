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
            Token::ARRAY => 36,
            Token::BOOLEAN => 37,
            Token::BREAK => 38,
            Token::CHAR => 39,
            Token::CONTINUE => 40,
            Token::DO => 41,
            Token::ELSE => 42,
            Token::FALSE => 43,
            Token::FUNCTION => 44,
            Token::IF => 45,
            Token::INTEGER => 46,
            Token::OF => 47,
            Token::RETURN => 48,
            Token::STRING => 49,
            Token::STRUCT => 50,
            Token::TRUE => 51,
            Token::TYPE => 52,
            Token::VAR => 53,
            Token::WHILE => 54,
            Token::COLON => 55,
            Token::SEMICOLON => 56,
            Token::COMMA => 57,
            Token::EQUAL => 58,
            Token::LEFTSQUARE => 59,
            Token::RIGHTSQUARE => 60,
            Token::LEFTBRACE => 61,
            Token::RIGHTBRACE => 62,
            Token::LEFTPARENTHESIS => 63,
            Token::RIGHTPARENTHESIS => 64,
            Token::AND => 65,
            Token::OR => 66,
            Token::LESSTHAN => 67,
            Token::GREATERTHAN => 68,
            Token::LESSOREQUAL => 69,
            Token::GREATEROREQUAL => 70,
            Token::NOTEQUAL => 71,
            Token::EQUALEQUAL => 72,
            Token::PLUS => 73,
            Token::PLUSPLUS => 74,
            Token::MINUS => 75,
            Token::MINUSMINUS => 76,
            Token::TIMES => 77,
            Token::DIVIDE => 78,
            Token::DOT => 79,
            Token::NOT => 80,
            Token::CHARACTER(_) => 81,
            Token::NUMERAL(_) => 82,
            Token::STRINGVAL(_) => 83,
            Token::ID(_) => 84,
            Token::DOLLAR => 85,
            Token::UNKNOWN(_) => 1000,
        }
    }
}
