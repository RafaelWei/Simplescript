pub mod constants;
pub mod rules;
pub mod terminals;
pub mod nonterminals;
pub mod token_stream;
pub mod scope_analyzer;

use token_stream::TokenStream;
use rules::Rules;
use scope_analyzer::ScopeAnalyzer;
use nonterminals::Object;

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

    pub fn semantics(&mut self, reduction_rule: Rules) {
        match reduction_rule {
            Rules::IDD => {
                let idd = nonterminals::AttribToken::IDD(Object(self.token_sec.clone()));
                if let Some(_) = self.scope_analyzer.search(self.token_sec.as_str()) {
                    dbg!("{:?}", &self.token_sec);
                    panic!("IDENTIFIER REDECLARATION",);
                } else {
                    self.scope_analyzer.define(self.token_sec.clone());
                    self.semantic_stack.push(idd);
                }
            },
            Rules::IDU => {
                dbg!("DENTRO DO IDU");
                let idu = nonterminals::AttribToken::IDU(Object(self.token_sec.clone()));
                if let Some(_) = self.scope_analyzer.find(self.token_sec.as_str()) {
                    self.semantic_stack.push(idu);
                } else {
                    panic!("IDENTIFIER NOT DECLARED");
                    //self.scope_analyzer.define(self.token_sec.clone());
                }
            },
            Rules::DF => {
                self.scope_analyzer.end_block();
            },
            Rules::DT_STRUCT => {
                self.scope_analyzer.end_block();
            },
            Rules::NF => {
                self.scope_analyzer.new_block();
            },
            Rules::NB => {
                self.scope_analyzer.new_block();
            },
            _ => ()
        }
    }
}
