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

#[derive(Clone, PartialEq, Eq)]
pub enum AttribToken {
    B,
    CHR(Object, char), 
    DC,
    DE,
    DF,
    DT,
    DV,
    E(Type),
    F(Type),
    FALSE(Object, bool),
    IDD(Object),
    IDU(Object),
    L(Type),
    LDE,
    LDV,
    LE(Vec<Type>, usize),
    LI(Vec<Object>),
    LP(Vec<Object>),
    LS,
    LV(Type),
    MC(Type, Vec<Type>),
    ME,
    MF,
    MT,
    MW,
    NB,
    NF,
    NUM(Object, i32),
    P,
    R(Type),
    S,
    STR(Object, String),
    T(Type),
    TRUE(Object, bool),
    Y(Type) 
}

#[derive(Clone, PartialEq, Eq)]
pub struct Object(pub String, pub Kind);

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Eq)]
pub enum Kind{
    no_kind_def,
    var(Type),
    param(Type),
    function(Type, Vec<Type>),
    field(Type),
    scalar(Type),
    array(Type),
    struct_(Type),
    alias(Type),
    universal
}

#[derive(Clone, PartialEq, Eq)]
pub struct StructField(pub String, pub Type);

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Int_,
    Char_,
    Bool_,
    String_,
    Array_type_(u32, Box<Type>),
    Alias_type_(Box<Type>),
    Struct_type_(Box<Vec<Object>>)
}
