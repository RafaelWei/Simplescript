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
        self.contexts.last_mut().unwrap().push(nonterminals::Object(new_identifier));
    }

    pub fn find(&self, target: &str) -> Option<nonterminals::Object>{
        for context in self.contexts.iter() {
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
}

