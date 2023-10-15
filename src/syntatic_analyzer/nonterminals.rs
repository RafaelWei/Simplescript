pub enum Token {
B, C, DC, DE, DF, DT, DV, E, F, FALSE, ID, IDD, IDU, L, LDE, LDV, LE, LI, LP, LS, LV, MC, ME, MF, MT, MW, NB, NF, NUM, P, R, S, STR, T, TRUE, Y, 
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
            Token::ID => 10,
            Token::IDD => 11,
            Token::IDU => 12,
            Token::L => 13,
            Token::LDE => 14,
            Token::LDV => 15,
            Token::LE => 16,
            Token::LI => 17,
            Token::LP => 18,
            Token::LS => 19,
            Token::LV => 20,
            Token::MC => 21,
            Token::ME => 22,
            Token::MF => 23,
            Token::MT => 24,
            Token::MW => 25,
            Token::NB => 26,
            Token::NF => 27,
            Token::NUM => 28,
            Token::P => 29,
            Token::R => 30,
            Token::S => 31,
            Token::STR => 32,
            Token::T => 33,
            Token::TRUE => 34,
            Token::Y => 35,
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
