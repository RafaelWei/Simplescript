use crate::token::*;

#[test]
fn test_colon(){
    let program = String::from(":");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::COLON)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_semicolon(){
    let program = String::from(";");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::SEMICOLON)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_comma(){
    let program = String::from(",");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::COMMA)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_equal(){
    let program = String::from("=");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::EQUAL)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_equalequal(){
    let program = String::from("==");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::EQUALEQUAL)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_leftsquare(){
    let program = String::from("[");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::LEFTSQUARE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_rightsquare(){
    let program = String::from("]");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::RIGHTSQUARE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_leftbrace(){
    let program = String::from("{");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::LEFTBRACE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_rightbrace(){
    let program = String::from("}");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::RIGHTBRACE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_leftparenthesis(){
    let program = String::from("(");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_rightparenthesis(){
    let program = String::from(")");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_and(){
    let program = String::from("&&");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::AND)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_or(){
    let program = String::from("||");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::OR)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_lessthan(){
    let program = String::from("<");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::LESSTHAN)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_lessorequal(){
    let program = String::from("<=");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::LESSOREQUAL)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_greaterthan(){
    let program = String::from(">");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::GREATERTHAN)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_greaterorequal(){
    let program = String::from(">=");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::GREATEROREQUAL)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_not(){
    let program = String::from("!");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::NOT)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_notequal(){
    let program = String::from("!=");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::NOTEQUAL)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_plus(){
    let program = String::from("+");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::PLUS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_plusplus(){
    let program = String::from("++");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::PLUSPLUS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_minus(){
    let program = String::from("-");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::MINUS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_minusminus(){
    let program = String::from("--");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::MINUSMINUS)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_times(){
    let program = String::from("*");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::TIMES)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_divide(){
    let program = String::from("/");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::DIVIDE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_dot(){
    let program = String::from(".");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::DOT)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_array(){
    let program = String::from("array");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::ARRAY)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_boolean(){
    let program = String::from("boolean");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::BOOLEAN)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_break(){
    let program = String::from("break");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::BREAK)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_char(){
    let program = String::from("char");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::CHAR)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_continue(){
    let program = String::from("continue");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::CONTINUE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_do(){
    let program = String::from("do");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::DO)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_function(){
    let program = String::from("function");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::FUNCTION)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_if(){
    let program = String::from("if");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::IF)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_else(){
    let program = String::from("else");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::ELSE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_integer(){
    let program = String::from("integer");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::INTEGER)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_of(){
    let program = String::from("of");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::OF)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_string(){
    let program = String::from("string");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::STRING)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_struct(){
    let program = String::from("struct");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::STRUCT)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_true(){
    let program = String::from("true");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::TRUE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_false(){
    let program = String::from("false");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::FALSE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_type(){
    let program = String::from("type");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::TYPE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_var(){
    let program = String::from("var");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::VAR)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_while(){
    let program = String::from("while");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::WHILE)); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_id(){
    let program = String::from("identifier");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::ID("identifier"))); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_character(){
    let program = String::from("'a'");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::CHARACTER('a'))); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_stringval(){
    let program = String::from(r#""Hello   World""#);
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::STRINGVAL("Hello   World"))); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_numeral(){
    let program = String::from("123.456");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::NUMERAL("123.456"))); 
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_unknown_from_and(){
    let program = String::from("&a");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::UNKNOWN("&")));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_unknown_from_or(){
    let program = String::from("|a");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::UNKNOWN("|")));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

/*
#[test]
fn test_unknown_from_end(){
    let program = String::from(r#""sdfdsfdsf"#);
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}
*/

/*
#[test]
fn test_unknown_from_character(){
    let program = String::from("'a");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::UNKNOWN));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}
*/


#[test]
fn test_mix_and_next_line() {
    let program = String::from("struct while() array if else && ||\ncontinue \ndo");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::STRUCT));
    assert_eq!(t_stream.next(), Some(Token::WHILE));
    assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::ARRAY));
    assert_eq!(t_stream.next(), Some(Token::IF));
    assert_eq!(t_stream.next(), Some(Token::ELSE));
    assert_eq!(t_stream.next(), Some(Token::AND));
    assert_eq!(t_stream.next(), Some(Token::OR));
    assert_eq!(t_stream.next(), Some(Token::CONTINUE));
    assert_eq!(t_stream.next(), Some(Token::DO));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}

#[test]
fn test_some_random() {
    let program = String::from("
                               if (a > b) {
                                   var tmp : integer;
                                   tmp = funcaoRecursiva(tmp-1);
                               }
                               ");
    let mut t_stream = TokenStream::new(program.chars());
    assert_eq!(t_stream.next(), Some(Token::IF));
    assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::ID("a")));
    assert_eq!(t_stream.next(), Some(Token::GREATERTHAN));
    assert_eq!(t_stream.next(), Some(Token::ID("b")));
    assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::LEFTBRACE));
    assert_eq!(t_stream.next(), Some(Token::VAR));
    assert_eq!(t_stream.next(), Some(Token::ID("tmp")));
    assert_eq!(t_stream.next(), Some(Token::COLON));
    assert_eq!(t_stream.next(), Some(Token::INTEGER));
    assert_eq!(t_stream.next(), Some(Token::SEMICOLON));
    assert_eq!(t_stream.next(), Some(Token::ID("tmp")));
    assert_eq!(t_stream.next(), Some(Token::EQUAL));
    assert_eq!(t_stream.next(), Some(Token::ID("funcaoRecursiva")));
    assert_eq!(t_stream.next(), Some(Token::LEFTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::ID("tmp")));
    assert_eq!(t_stream.next(), Some(Token::MINUS));
    assert_eq!(t_stream.next(), Some(Token::NUMERAL("1")));
    assert_eq!(t_stream.next(), Some(Token::RIGHTPARENTHESIS));
    assert_eq!(t_stream.next(), Some(Token::SEMICOLON));
    assert_eq!(t_stream.next(), Some(Token::RIGHTBRACE));
    assert_eq!(t_stream.next(), Some(Token::DOLLAR));
}
