use std::collections::HashMap;
use std::fmt;
use super::token_types;

pub enum StackType {
    Number(i64),
    Text(String),
    Bool(bool),
}

impl fmt::Debug for StackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StackType")
            .finish()
    }
}

pub enum EnvType {
    Number(i64),
    Text(String),
    NumberVector(Vec<i64>),
    TextVector(Vec<String>),
    NumberMap(HashMap<String, i64>),
    TextMap(HashMap<String, String>),
}

pub struct Context<'a> {
    the_stack: Vec<StackType>,
    the_environment: HashMap<String, &'a EnvType>,
    the_problems: Vec<String>,
    branches: Vec<(token_types::TokenTypes, bool)>,
    program_counter: usize,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            the_stack: Vec::new(),
            the_environment: HashMap::new(),
            the_problems: Vec::new(),
            branches: Vec::new(),
            program_counter: 0,
        }
    }

    pub fn branch_is_taken(&self) -> &(token_types::TokenTypes, bool) {
        self.branches.last().unwrap_or(&(token_types::TokenTypes::INVALID, true))
    }

    pub fn branch_push(&mut self, ttype: token_types::TokenTypes, cond: bool) {
        self.branches.push((ttype, cond));
    }

    pub fn branch_pop(&mut self) -> Result<(), ()> {
        match self.branches.pop() {
            Some(_) => Ok(()),
            None => Err(()),
        }
    }

    pub fn errors_push(&mut self, error: String) {
        self.the_problems.push(error);
    }

    pub fn errors_it(&self) -> std::slice::Iter<String> {
        self.the_problems.iter()
    }

    pub fn stack_push(&mut self, data: StackType) {
        self.the_stack.push(data);
    }

    pub fn stack_pop(&mut self) -> Option<StackType> {
        self.the_stack.pop()
    }

    pub fn env_add_or_update(&mut self, key: &String, data: &'a EnvType) {
        self.the_environment.entry(key.clone()).or_insert(data);
    }

    pub fn env_get(&self, key: &String) -> Option<&'a EnvType> {
        match self.the_environment.get(key) {
            Some(data) => Some(*data),
            None => None,
        }
    }

    pub fn env_key_exists(&self, key: &String) -> bool {
        self.the_environment.contains_key(key)
    }

    pub fn stack_push_from_env(&mut self, key: &String) {
        if !self.env_key_exists(key) {
            return;
        }

        match self.the_environment.get(key).unwrap() {
            EnvType::Number(n) => self.the_stack.push(StackType::Number(n.clone())),
            EnvType::Text(t) => self.the_stack.push(StackType::Text(t.clone())),
            _ => self.errors_push(String::from("complex variable, must be parsed")),
        }
    }

    pub fn stack_push_from_env_vector(&mut self, key: &String, id: usize) {
        if !self.env_key_exists(key) {
            return;
        }

        match self.the_environment.get(key).unwrap() {
            EnvType::TextVector(v) => {
                match v.get(id) {
                    Some(t) => self.the_stack.push(StackType::Text(t.clone())),
                    None => panic!("invalid id {}[{}]", key, id),
                }
            },
            EnvType::NumberVector(v) => {
                match v.get(id) {
                    Some(t) => self.the_stack.push(StackType::Number(t.clone())),
                    None => panic!("invalid id {}[{}]", key, id),
                }
            },
            _ => panic!("complex variable, must be parsed"),
        }
    }

    pub fn stack_push_from_env_map(&mut self, key: &String, id: &String) {
        if !self.env_key_exists(key) {
            return;
        }

        match self.the_environment.get(key).unwrap() {
            EnvType::TextMap(v) => {
                match v.get(id) {
                    Some(t) => self.the_stack.push(StackType::Text(t.clone())),
                    None => panic!("invalid id {}[{}]", key, id),
                }
            },
            EnvType::NumberMap(v) => {
                match v.get(id) {
                    Some(t) => self.the_stack.push(StackType::Number(t.clone())),
                    None => panic!("invalid id {}[{}]", key, id),
                }
            },
            _ => panic!("complex variable, must be parsed"),
        }
    }
}

impl<'a> fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Token")
            .field("the_stack", &self.the_stack)
            .field("program_counter", &self.program_counter)
            .finish()
    }
}
