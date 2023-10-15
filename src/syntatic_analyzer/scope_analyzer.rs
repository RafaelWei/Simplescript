use crate::syntatic_analyzer::nonterminals;

pub struct ScopeAnalyzer {
    contexts: Vec<Vec<nonterminals::Object>>
}

impl ScopeAnalyzer {
    pub fn new() -> Self {
        ScopeAnalyzer {
            contexts: vec![vec![]]
        }
    }
    pub fn new_block(&mut self) {
        self.contexts.push(vec![]);
    }

    pub fn end_block(&mut self) {
        let new_size = self.contexts.len() - 1;
        self.contexts.truncate(new_size);
    }

    pub fn define(&mut self, new_identifier: String) {
        self.contexts.last_mut().unwrap().push(nonterminals::Object(new_identifier, nonterminals::Kind::no_kind_def));
    }

    pub fn find(&self, target: &str) -> Option<nonterminals::Object>{
        for context in self.contexts.iter().rev() {
            for element in context.iter() {
                if element.0.as_str() == target {
                    return Some(element.clone());
                }
            }
        }
        return None;
    }

    pub fn search(&self, target: &str) -> Option<nonterminals::Object>{
        for element in self.contexts.last().unwrap().iter() {
            if element.0.as_str() == target {
                return Some(element.clone());
            }
        }
        return None;
    }

    pub fn var_decl(&mut self, target: &str, var_type: nonterminals::Type) {
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                *element = nonterminals::Object(element.0.clone(), nonterminals::Kind::var(var_type.clone()));
            }
        }
    }

    pub fn array_type_decl(&mut self, target: &str, n_elements: i32, elements_type: nonterminals::Type) {
        if n_elements < 0 {
            panic!("ARRAY SIZE MUST BE POSITIVE");
        }
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                let u32_n_elements = u32::try_from(n_elements).ok().unwrap();
                *element = nonterminals::Object(element.0.clone(), 
                                                nonterminals::Kind::array(nonterminals::Type::Array_type_(u32_n_elements, Box::new(elements_type))));
                break;
            }
        }
    }

    pub fn alias_type_decl(&mut self, target: &str, alias_type: nonterminals::Type) {
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                *element = nonterminals::Object(element.0.clone(), nonterminals::Kind::alias(alias_type.clone()));
            }
        }
    }

    pub fn struct_field_decl(&mut self, target: &str, field_type: nonterminals::Type) {
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                *element = nonterminals::Object(element.0.clone(), nonterminals::Kind::field(field_type.clone()));
            }
        }
    }

    pub fn struct_type_decl(&mut self, target: &str) {
        // the top context is the one with the objects representing the struct fields
        let struct_fields = Box::new(self.contexts.pop().unwrap());
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                *element = nonterminals::Object(element.0.clone(), nonterminals::Kind::struct_(nonterminals::Type::Struct_type_(struct_fields))); 
                break;
            }
        }
    }

    pub fn param_type_decl(&mut self, target: &str, param_type: nonterminals::Type) {
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                *element = nonterminals::Object(element.0.clone(), nonterminals::Kind::param(param_type.clone()));
            }
        }
    }

    pub fn func_type_decl(&mut self, target: &str, return_type: nonterminals::Type, param_types: Vec<nonterminals::Type>) {
        let func_context = self.contexts.pop().unwrap();
        for element in self.contexts.last_mut().unwrap().iter_mut() {
            if element.0.as_str() == target {
                element.1 = nonterminals::Kind::function(return_type.clone(), param_types.clone());
            }
        }
        self.contexts.push(func_context);
    }
}

