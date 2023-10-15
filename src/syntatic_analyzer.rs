pub mod constants;
pub mod rules;
pub mod terminals;
pub mod nonterminals;
pub mod token_stream;
pub mod scope_analyzer;

use token_stream::TokenStream;
use rules::Rules;
use scope_analyzer::ScopeAnalyzer;
use nonterminals::{Object, Kind};

pub struct StateMachine {
    program: String,
    token_sec: String,
    syntatic_stack: Vec<usize>, 
    semantic_stack: Vec<nonterminals::AttribToken>,
    scope_analyzer: ScopeAnalyzer
}

impl StateMachine {
    pub fn new(program: String) -> Self {
        StateMachine{
            program,
            token_sec: String::new(), 
            syntatic_stack: vec![0; 1],
            semantic_stack: vec![],
            scope_analyzer: ScopeAnalyzer::new()
        }
    }

    
    pub fn parse(&mut self) -> bool {
        let temp = self.program.clone();
        let mut token_stream = TokenStream::new(temp.chars());
        let mut curr_tok: terminals::Token;
        let mut action: i32;
        let mut reduction_rule: usize;


        curr_tok = token_stream.next().unwrap();
        

        loop{
            action = constants::ACTIONTABLE[*self.syntatic_stack.last().unwrap()][usize::from(curr_tok.clone())];

            match curr_tok {
                terminals::Token::ID(id) => {
                    self.token_sec = id.to_string();
                },
                terminals::Token::CHARACTER(ch) => {
                    self.token_sec = String::from(ch);
                },
                terminals::Token::STRINGVAL(str_val) => {
                    self.token_sec = str_val.to_string();
                },
                terminals::Token::NUMERAL(num) => {
                    self.token_sec = num.to_string();
                },
                _ => ()
            }

            dbg!(&curr_tok);
            if action > 0 {
                self.syntatic_stack.push(usize::try_from(action).unwrap());
                curr_tok = token_stream.next().unwrap();
            }
            else if action < 0 {
                /* 
                 * Popping the corresponding states. There is a one-unit padding in the rule
                 * enumeration because we need to start counting the rules from 1 as 0's in the
                 * table means syntax error and the indexing in arrays starts from 0. That's why
                 * reduce_rule = action - 1.
                 */

                action = -action;
                reduction_rule = usize::try_from(action-1).unwrap();
                dbg!(Rules::from(reduction_rule));
                self.semantics(Rules::from(reduction_rule));
                let new_length = self.syntatic_stack.len() - constants::RULELEN[reduction_rule];
                self.syntatic_stack.truncate(new_length);

                // pushing the state after the transition with the non terminal
                self.syntatic_stack.push(usize::try_from(
                                    constants::ACTIONTABLE
                                    [*self.syntatic_stack.last().unwrap()]
                                    [constants::RULELEFT[reduction_rule].into_usize()]).unwrap());
            }
            else {
                panic!("SYNTAX ERROR\n");
            }
            
            if *self.syntatic_stack.last().unwrap() == 1 {
                println!("FINISHED PARSING\n");
                return true;
            }
        }
    }

    fn check_types(&self, t1: nonterminals::Type, t2: nonterminals::Type) -> bool {
        if t1 == t2 {
            return true;
        }

        return false;
    }

    pub fn semantics(&mut self, reduction_rule: Rules) {
        match reduction_rule {
            Rules::IDD => {
                let idd = nonterminals::AttribToken::IDD(Object(self.token_sec.clone(), nonterminals::Kind::no_kind_def));
                if let Some(_) = self.scope_analyzer.search(self.token_sec.as_str()) {
                    dbg!("{:?}", &self.token_sec);
                    panic!("IDENTIFIER REDECLARATION");
                } else {
                    self.scope_analyzer.define(self.token_sec.clone());
                    self.semantic_stack.push(idd);
                }
            },
            Rules::IDU => {
                if let Some(obj) = self.scope_analyzer.find(self.token_sec.as_str()) {
                    let idu = nonterminals::AttribToken::IDU(obj);
                    self.semantic_stack.push(idu);
                } else {
                    panic!("IDENTIFIER NOT DECLARED");
                    //self.scope_analyzer.define(self.token_sec.clone());
                }
            },
            Rules::NF => {
                self.scope_analyzer.new_block();
                self.semantic_stack.push(nonterminals::AttribToken::NF);
            },
            Rules::NB => {
                self.scope_analyzer.new_block();
                self.semantic_stack.push(nonterminals::AttribToken::NB);
            },
            Rules::T_INT => {
                let t = nonterminals::AttribToken::T(nonterminals::Type::Int_);
                self.semantic_stack.push(t);
            },
            Rules::T_BOOL => {
                let t = nonterminals::AttribToken::T(nonterminals::Type::Bool_);
                self.semantic_stack.push(t);
            },
            Rules::T_CHAR => {
                let t = nonterminals::AttribToken::T(nonterminals::Type::Char_);
                self.semantic_stack.push(t);
            },
            Rules::T_STRING => {
                let t = nonterminals::AttribToken::T(nonterminals::Type::String_);
                self.semantic_stack.push(t);
            },
            Rules::T_IDU => {
                let idu = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::IDU(obj) = idu {
                    match obj.1 {
                        nonterminals::Kind::scalar(type_) => {
                            let t = nonterminals::AttribToken::T(type_);
                            self.semantic_stack.push(t);
                        },
                        nonterminals::Kind::array(type_) => {
                            let t = nonterminals::AttribToken::T(type_);
                            self.semantic_stack.push(t); 
                        },
                        nonterminals::Kind::struct_(type_) => {
                            let t = nonterminals::AttribToken::T(type_);
                            self.semantic_stack.push(t);
                        },
                        nonterminals::Kind::alias(type_) => {
                            let t = nonterminals::AttribToken::T(type_);
                            self.semantic_stack.push(t);
                        }
                        _ => {
                            panic!("EXPECTED IDENTIFIER WITH KIND TYPE");
                        }
                    }
                }
            },
            Rules::LI_IDD => {
                let idd = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::IDD(obj) = idd {
                    let li = nonterminals::AttribToken::LI(vec![obj]);
                    self.semantic_stack.push(li);
                }
            },
            Rules::LI_LI => {
                let idd = self.semantic_stack.pop().unwrap();
                let li1 = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::IDD(obj) = idd {
                    if let nonterminals::AttribToken::LI(mut vec_obj) = li1 {
                        vec_obj.push(obj);
                        let li0 = nonterminals::AttribToken::LI(vec_obj);
                        self.semantic_stack.push(li0);
                    }
                }
            },
            Rules::DV => {
                let t = self.semantic_stack.pop().unwrap();
                let li = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LI(obj_vec) = li {
                    if let nonterminals::AttribToken::T(type_) = t {
                        for obj in obj_vec.iter() {
                            self.scope_analyzer.var_decl(obj.0.as_str(), type_.clone());
                        }
                    }
                }
                self.semantic_stack.push(nonterminals::AttribToken::DV);
            },
            Rules::TRUE => {
                let true_ = nonterminals::AttribToken::TRUE(Object(String::new(), Kind::scalar(nonterminals::Type::Bool_)), true);
                self.semantic_stack.push(true_);
            },
            Rules::FALSE => {
                let false_ = nonterminals::AttribToken::FALSE(Object(String::new(), Kind::scalar(nonterminals::Type::Bool_)), false);
                self.semantic_stack.push(false_);
            },
            Rules::C => {
                let ch = self.token_sec.chars().nth(0).unwrap();
                let char_ = nonterminals::AttribToken::CHR(Object(String::new(), Kind::scalar(nonterminals::Type::Char_)), ch);
                self.semantic_stack.push(char_);
            },
            Rules::STR => {
                let string_ = nonterminals::AttribToken::STR(Object(String::new(), Kind::scalar(nonterminals::Type::String_)), self.token_sec.clone());
                self.semantic_stack.push(string_);
            },
            Rules::NUM => {
                let num_ = nonterminals::AttribToken::NUM(Object(String::new(), Kind::scalar(nonterminals::Type::Int_)), self.token_sec.parse::<i32>().unwrap());
                self.semantic_stack.push(num_);
            },
            Rules::DT_ARRAY => {
                let t = self.semantic_stack.pop().unwrap();
                let num = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::NUM(_, n_elements) = num {
                        if let nonterminals::AttribToken::IDD(obj) = idd {
                            let name = obj.0;
                            self.scope_analyzer.array_type_decl(name.as_str(), n_elements, type_.clone());
                            self.semantic_stack.push(nonterminals::AttribToken::DT);
                        }
                    }
                }
            },
            Rules::DT_ALIAS => {
                let t = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::IDD(obj) = idd {
                        let name = obj.0;
                        self.scope_analyzer.alias_type_decl(name.as_str(), type_.clone());
                        self.semantic_stack.push(nonterminals::AttribToken::DT);
                    }
                }
            },
            Rules::DC_LI => {
                let t = self.semantic_stack.pop().unwrap();
                let li = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::LI(obj_vec) = li {
                        for obj in obj_vec.iter() {
                            let name = obj.0.clone();
                            self.scope_analyzer.struct_field_decl(name.as_str(), type_.clone());
                        }
                        let dc = nonterminals::AttribToken::DC;
                        self.semantic_stack.push(dc);
                    }
                }
            },
            Rules::DC_DC => {
                let t = self.semantic_stack.pop().unwrap();
                let li = self.semantic_stack.pop().unwrap();
                let _dc1 = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::LI(obj_vec) = li {
                        //for obj in li_obj_vec.iter_mut() {
                        //    obj.1 = Kind::field(type_.clone());
                        //}
                        for obj in obj_vec.iter() {
                            let name = obj.0.clone();
                            self.scope_analyzer.struct_field_decl(name.as_str(), type_.clone());
                        }
                        //if let nonterminals::AttribToken::DC(mut dc_obj_vec) = dc1 {
                        //    dc_obj_vec.append(li_obj_vec);
                        //    let dc0 = nonterminals::AttribToken::DC(dc_obj_vec);
                        //    self.semantic_stack.push(dc0);
                       // }
                        let dc0 = nonterminals::AttribToken::DC;
                        self.semantic_stack.push(dc0);
                    }
                }
            },
            Rules::DT_STRUCT => {
                let dc = self.semantic_stack.pop().unwrap();
                let _nb = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::DC = dc {
                    if let nonterminals::AttribToken::IDD(idd_obj) = idd {
                        let name = idd_obj.0;
                        //let mut type_vec: Vec<nonterminals::Type> = vec![];
                        //for obj in obj_vec.iter() {
                        //    if let Kind::field(type_) = &obj.1 {
                        //        type_vec.push(type_.clone());
                        //    }
                        //}
                        self.scope_analyzer.struct_type_decl(name.as_str());
                        self.semantic_stack.push(nonterminals::AttribToken::DT);
                    }
                }
            },
            Rules::LP_IDD => {
                let t = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::IDD(mut obj) = idd {
                        let name = obj.0.clone();
                        self.scope_analyzer.param_type_decl(name.as_str(), type_.clone());

                        obj.1 = Kind::param(type_.clone());
                        let lp = nonterminals::AttribToken::LP(vec![obj]);
                        self.semantic_stack.push(lp);
                    }
                }
            },
            Rules::LP_LP => {
                let t = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();
                let lp1 = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t {
                    if let nonterminals::AttribToken::IDD(mut obj) = idd {
                        if let nonterminals::AttribToken::LP(mut obj_vec) = lp1 {
                            let name = obj.0.clone();
                            self.scope_analyzer.param_type_decl(name.as_str(), type_.clone());

                            obj.1 = Kind::param(type_.clone());
                            obj_vec.push(obj);
                            let lp0 = nonterminals::AttribToken::LP(obj_vec);
                            self.semantic_stack.push(lp0);
                        }
                    }
                }
            },
            Rules::MF => {
                let t = self.semantic_stack.pop().unwrap();
                let lp = self.semantic_stack.pop().unwrap();
                let _nb = self.semantic_stack.pop().unwrap();
                let idd = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::T(type_) = t.clone() {
                    if let nonterminals::AttribToken::LP(obj_vec) = lp.clone() {
                        if let nonterminals::AttribToken::IDD(idd_obj) = idd.clone() {
                            // get the array of types out of the array of objects
                            let mut types_vec = vec![];
                            for obj in obj_vec.iter() {
                                if let nonterminals::Kind::param(type_) = obj.1.clone() {
                                    types_vec.push(type_);
                                }
                            }

                            self.scope_analyzer.func_type_decl(idd_obj.0.as_str(), type_, types_vec);

                            self.semantic_stack.push(idd);
                            self.semantic_stack.push(_nb);
                            self.semantic_stack.push(lp);
                            self.semantic_stack.push(t);
                            self.semantic_stack.push(nonterminals::AttribToken::MF);
                        }
                    }
                }
            },
            Rules::DF => {
                let _b = self.semantic_stack.pop();
                let _mf = self.semantic_stack.pop();
                let _t = self.semantic_stack.pop();
                let _lp = self.semantic_stack.pop();
                let _nb = self.semantic_stack.pop();
                let _idd = self.semantic_stack.pop();

                self.semantic_stack.push(nonterminals::AttribToken::DF);
                self.scope_analyzer.end_block();
            },
            Rules::S_NB => {
                self.scope_analyzer.end_block();
                self.semantic_stack.push(nonterminals::AttribToken::S);
            },
            Rules::S_IF => {
                let _s = self.semantic_stack.pop();
                let e = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::E(type_) = e {
                    if !self.check_types(type_, nonterminals::Type::Bool_) {
                        panic!("IF STATEMENT EXPECTS A BOOLEAN TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::S);
                }
            },
            Rules::S_IF_ELSE => {
                let _s2 = self.semantic_stack.pop();
                let _s1 = self.semantic_stack.pop();
                let e = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::E(type_) = e {
                    if !self.check_types(type_, nonterminals::Type::Bool_) {
                        panic!("IF ELSE STATEMENT EXPECTS A BOOLEAN TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::S);
                }
            },
            Rules::S_WHILE => {
                let _s = self.semantic_stack.pop();
                let e = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::E(type_) = e {
                    if !self.check_types(type_, nonterminals::Type::Bool_) {
                        panic!("WHILE STATEMENT EXPECTS A BOOLEAN TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::S);
                }
            },
            Rules::S_DO_WHILE => {
                let e = self.semantic_stack.pop().unwrap();
                let _s = self.semantic_stack.pop();
                if let nonterminals::AttribToken::E(type_) = e {
                    if !self.check_types(type_, nonterminals::Type::Bool_) {
                        panic!("WHILE STATEMENT EXPECTS A BOOLEAN TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::S);
                }
            },
            Rules::S_LV => {
                let e = self.semantic_stack.pop().unwrap();
                let lv = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::E(type_e) = e {
                    if let nonterminals::AttribToken::LV(type_lv) = lv {
                        if !self.check_types(type_e, type_lv) {
                            panic!("ASSINGMENT TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::S);
                    }
                }
            },
            Rules::E_AND => {
                let l = self.semantic_stack.pop().unwrap();
                let e = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::L(type_l) = l {
                    if let nonterminals::AttribToken::E(type_e) = e {
                        if !self.check_types(type_l, nonterminals::Type::Bool_) {
                            panic!("AND EXPRESSION FIRST ELEMENT SHOULD BE BOOL TYPE");
                        }
                        if !self.check_types(type_e, nonterminals::Type::Bool_) {
                            panic!("AND EXPRESSION SECOND ELEMENT SHOULD BE BOOL TYPE");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::E(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::E_OR => {
                let l = self.semantic_stack.pop().unwrap();
                let e = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::L(type_l) = l {
                    if let nonterminals::AttribToken::E(type_e) = e {
                        if !self.check_types(type_l, nonterminals::Type::Bool_) {
                            panic!("OR EXPRESSION FIRST ELEMENT SHOULD BE BOOL TYPE");
                        }
                        if !self.check_types(type_e, nonterminals::Type::Bool_) {
                            panic!("OR EXPRESSION SECOND ELEMENT SHOULD BE BOOL TYPE");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::E(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::E_L => {
                let l = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::L(type_) = l {
                    self.semantic_stack.push(nonterminals::AttribToken::E(type_));
                }
            },
            Rules::L_LESS_THAN => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("LESS THAN COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_GREATER_THAN => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("GREATER THAN COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_LESS_EQUAL => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("LESS OR EQUAL COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_GREATER_EQUAL => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("GREATER OR EQUAL COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_EQUAL_EQUAL => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("EQUAL EQUAL COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_NOT_EQUAL => {
                let r = self.semantic_stack.pop().unwrap();
                let l = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::R(type_r) = r {
                    if let nonterminals::AttribToken::L(type_l) = l {
                        if !self.check_types(type_r, type_l) {
                            panic!("NOT EQUAL COMPARISONS TYPE MISMATCH");
                        }
                        self.semantic_stack.push(nonterminals::AttribToken::L(nonterminals::Type::Bool_));
                    }
                }
            },
            Rules::L_R => {
                let r = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::R(type_) = r {
                    self.semantic_stack.push(nonterminals::AttribToken::L(type_));
                }
            },
            Rules::R_PLUS => {
                let y = self.semantic_stack.pop().unwrap();
                let r = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::Y(type_y) = y {
                    if let nonterminals::AttribToken::R(type_r) = r {
                        if !self.check_types(type_r.clone(), type_y) {
                            panic!("ADDITION TYPE MISMATCH");
                        }

                        if !(self.check_types(type_r.clone(), nonterminals::Type::Int_) || 
                             self.check_types(type_r.clone(), nonterminals::Type::String_)) {
                            panic!("ADDITION ONLY TAKES INTEGER OR STRING ARGS");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::R(type_r));
                    }
                }
            },
            Rules::R_MINUS => {
                let y = self.semantic_stack.pop().unwrap();
                let r = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::Y(type_y) = y {
                    if let nonterminals::AttribToken::R(type_r) = r {
                        if !self.check_types(type_r.clone(), type_y) {
                            panic!("SUBTRACTION TYPE MISMATCH");
                        }

                        if !self.check_types(type_r.clone(), nonterminals::Type::Int_) {
                            panic!("SUBTRACTION ONLY TAKES INTEGER ARGS");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::R(type_r));
                    }
                }
            },
            Rules::R_Y => {
                let y = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::Y(type_) = y {
                    self.semantic_stack.push(nonterminals::AttribToken::R(type_));
                }
            },
            Rules::Y_TIMES => {
                let f = self.semantic_stack.pop().unwrap();
                let y = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::F(type_f) = f {
                    if let nonterminals::AttribToken::Y(type_y) = y {
                        if !self.check_types(type_f, type_y.clone()) {
                            panic!("MULTIPLICATION TYPE MISMATCH");
                        }

                        if !self.check_types(type_y.clone(), nonterminals::Type::Int_) {
                            panic!("MULTIPLICATION ONLY TAKES INT TYPES");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::Y(type_y));
                    }
                }
            },
            Rules::Y_DIVIDE => {
                let f = self.semantic_stack.pop().unwrap();
                let y = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::F(type_f) = f {
                    if let nonterminals::AttribToken::Y(type_y) = y{
                        if !self.check_types(type_f, type_y.clone()) {
                            panic!("DIVISION TYPE MISMATCH");
                        }

                        if !self.check_types(type_y.clone(), nonterminals::Type::Int_) {
                            panic!("DIVISION ONLY TAKES INT TYPES");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::Y(type_y));
                    }
                }
            },
            Rules::Y_F => {
                let f = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::F(type_) = f{
                    self.semantic_stack.push(nonterminals::AttribToken::Y(type_));
                }
            },
            Rules::F_LV => {
                let lv = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LV(type_) = lv {
                    self.semantic_stack.push(nonterminals::AttribToken::F(type_));
                }
            },
            Rules::F_PLUSPLUS => {
                let lv = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LV(type_) = lv {
                    if !self.check_types(type_, nonterminals::Type::Int_) {
                        panic!("LEFT INCREMENT INVALID TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
                }
            },
            Rules::F_MINUSMINUS => {
                let lv = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LV(type_) = lv {
                    if !self.check_types(type_, nonterminals::Type::Int_) {
                        panic!("LEFT DECREMENT INVALID TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
                }
            },
            Rules::F_LV_PLUSPLUS => {
                let lv = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LV(type_) = lv {
                    if !self.check_types(type_, nonterminals::Type::Int_) {
                        panic!("RIGHT INCREMENT INVALID TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
                }
            },
            Rules::F_LV_MINUSMINUS => {
                let lv = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::LV(type_) = lv {
                    if !self.check_types(type_, nonterminals::Type::Int_) {
                        panic!("RIGHT DECREMENT INVALID TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
                }
            },
            Rules::F_PAR_E_PAR => {
                let e = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::E(type_) = e {
                    self.semantic_stack.push(nonterminals::AttribToken::F(type_));
                }
            },
            Rules::F_NEGATIVE => {
                let f = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::F(type_) = f {
                    if !self.check_types(type_, nonterminals::Type::Int_) {
                        panic!("NEGATIVE OPERATOR ONLY TAKES INT TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
                }
            },
            Rules::F_NOT => {
                let f = self.semantic_stack.pop().unwrap();
                if let nonterminals::AttribToken::F(type_) = f {
                    if !self.check_types(type_, nonterminals::Type::Bool_) {
                        panic!("NOT OPERATOR ONLY TAKES BOOL TYPE");
                    }
                    self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Bool_));
                }
            },
            Rules::F_TRUE => {
                self.semantic_stack.pop();
                self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Bool_));
            },
            Rules::F_FALSE => {
                self.semantic_stack.pop();
                self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Bool_));
            },
            Rules::F_C => {
                self.semantic_stack.pop();
                self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Char_));
            },
            Rules::F_STR => {
                self.semantic_stack.pop();
                self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::String_));
            },
            Rules::F_NUM => {
                self.semantic_stack.pop();
                self.semantic_stack.push(nonterminals::AttribToken::F(nonterminals::Type::Int_));
            },
            Rules::LV_STRUCT => {
                let id = self.semantic_stack.pop().unwrap();
                let lv1 = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::IDU(id_obj) = id {
                    if let nonterminals::AttribToken::LV(type_) = lv1 {
                        if let nonterminals::Type::Struct_type_(boxed_obj_vec) = type_ {
                            // We need to look for the id in the struct type. If we don't find it,
                            // we must error out. We can change the struct type to hold a vec of
                            // other types later.
                            let mut found_field = false;
                            for obj in boxed_obj_vec.iter() {
                                 if id_obj.0 == obj.0 {
                                    if let Kind::field(field_type) = obj.1.clone() {
                                        self.semantic_stack.push(nonterminals::AttribToken::LV(field_type));    
                                        found_field = true;
                                        break;
                                    }
                                 }
                            }
                            if !found_field {
                                panic!("STRUCT FIELD NOT DECLARED");
                            }
                        }
                        else {
                            panic!("ERROR KIND IS NOT STRUCT");
                        }
                    }
                }
            },
            Rules::LV_ARR => {
                let e = self.semantic_stack.pop().unwrap();
                let lv1 = self.semantic_stack.pop().unwrap();
                let lv0: nonterminals::AttribToken;

                if let nonterminals::AttribToken::E(type_e) = e {
                    if let nonterminals::AttribToken::LV(type_lv) = lv1 {
                        if let nonterminals::Type::String_ = type_lv {
                            lv0 = nonterminals::AttribToken::LV(nonterminals::Type::Char_);    
                        }
                        else if let nonterminals::Type::Array_type_(_, type_) = type_lv {
                            lv0 = nonterminals::AttribToken::LV(*type_);
                        }
                        else {
                            panic!("INDEXED ACCESS ONLY TAKES STRING OR ARRAY ELEMENTS");
                        }

                        if !self.check_types(type_e, nonterminals::Type::Int_) {
                            panic!("INDEXES MUST HAVE INTEGER TYPE");
                        }

                        self.semantic_stack.push(lv0);
                    }
                }
            },
            Rules::LV_IDU => {
                let idu = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::IDU(obj) = idu {
                    if let nonterminals::Kind::var(type_) | nonterminals::Kind::param(type_) = obj.1 {
                        let lv = nonterminals::AttribToken::LV(type_);
                        self.semantic_stack.push(lv);
                    }
                    else {
                        panic!("ERROR LEFT VALUE KIND IS NOT VAR OR PARAM");
                    }
                }
            },
            Rules::MC => {
                let idu = self.semantic_stack.last().unwrap();

                if let nonterminals::AttribToken::IDU(obj) = idu {
                    if let nonterminals::Kind::function(ret_type, params_types_vec) = obj.1.clone() {
                        let mc = nonterminals::AttribToken::MC(ret_type, params_types_vec);
                        self.semantic_stack.push(mc);
                    }
                    else {
                        panic!("ERROR KIND IS NOT FUNCTION");
                    }
                }
            },
            Rules::LE_E => {
                let e = self.semantic_stack.pop().unwrap();
                
                if let nonterminals::AttribToken::E(type_) = e {
                    let mc = self.semantic_stack.last().unwrap();
                    if let nonterminals::AttribToken::MC(_, params_type_vec) = mc {
                        // if we there is no such first parameter, we must error out
                        let first_type = params_type_vec.get(0);
                        if first_type.is_none() {
                            panic!("EXPECTED NO ARGUMENTS");
                        }

                        if !self.check_types(type_, first_type.unwrap().clone()) {
                            panic!("ERROR TYPE PARAM 1");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::LE(params_type_vec.clone(), 1));
                    }
                }
            },
            Rules::LE_LE => {
                let e = self.semantic_stack.pop().unwrap();
                let le1 = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::E(type_) = e {
                    if let nonterminals::AttribToken::LE(params_type_vec, cnt) = le1 {
                        let nth_type = params_type_vec.get(cnt);
                        if nth_type.is_none() {
                            panic!("TOO MANY ARGUMENTS");
                        }
                        
                        if !self.check_types(type_, nth_type.unwrap().clone()) {
                            panic!("ERROR TYPE PARAM {:?}", cnt+1);
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::LE(params_type_vec, cnt+1));
                    }
                }
            },
            Rules::F_FUNC_CALL => {
                let le = self.semantic_stack.pop().unwrap();
                let mc = self.semantic_stack.pop().unwrap();
                let _idu = self.semantic_stack.pop().unwrap();

                if let nonterminals::AttribToken::MC(ret_type, _) = mc {
                    if let nonterminals::AttribToken::LE(params_type_vec, cnt) = le {
                        if params_type_vec.len() > cnt {
                            panic!("ERROR TOO FEW ARGUMENTS");
                        }

                        self.semantic_stack.push(nonterminals::AttribToken::F(ret_type));
                    }
                }
            },
            _ => ()
        }
    }
}
