pub enum Token {
B, C, DC, DE, DF, DT, DV, E, F, FALSE, IDD, IDU, L, LDE, LDV, LE, LI, LP, LS, LV, MC, ME, MF, MT, MW, NB, NF, NUM, P, R, S, STR, T, TRUE, Y, 
}

impl Token {
    pub fn into_usize(&self) -> usize {
        match self{
            Token::B => 0,
            Token::C => 1,
            Token::DC => 2,
            Token::DE => 3,
            Token::DF => 4,
            Token::DT => 5,
            Token::DV => 6,
            Token::E => 7,
            Token::F => 8,
            Token::FALSE => 9,
            Token::IDD => 10,
            Token::IDU => 11,
            Token::L => 12,
            Token::LDE => 13,
            Token::LDV => 14,
            Token::LE => 15,
            Token::LI => 16,
            Token::LP => 17,
            Token::LS => 18,
            Token::LV => 19,
            Token::MC => 20,
            Token::ME => 21,
            Token::MF => 22,
            Token::MT => 23,
            Token::MW => 24,
            Token::NB => 25,
            Token::NF => 26,
            Token::NUM => 27,
            Token::P => 28,
            Token::R => 29,
            Token::S => 30,
            Token::STR => 31,
            Token::T => 32,
            Token::TRUE => 33,
            Token::Y => 34,
        }
    }
}

#[derive(Clone)]
pub struct Object(pub String);

#[derive(Clone)]
pub enum AttribToken {
    IDD(Object),
    IDU(Object),
}

/*
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Kind{
    no_kind_def,
    var(Type),
    param(Type),
    function(Type, Vec<Type>),
    field(Type),
    array(u32, Type),
    struct_(Vec<Type>),
    alias(Type),
    universal
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub enum Type {
    Int_,
    Char_,
    Bool_,
    String_,
    Alias_type_(Box<Type>),
    Struct_type_(Vec<Box<Type>>)
}
*/
