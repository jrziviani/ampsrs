use std::collections::HashMap;
use std::fmt;

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
    program_counter: usize,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            the_stack: Vec::new(),
            the_environment: HashMap::new(),
            the_problems: Vec::new(),
            program_counter: 0,
        }
    }

    pub fn errors_push(&mut self, error: String) {
        self.the_problems.push(error);
    }

    pub fn stack_push(&mut self, data: StackType) {
        self.the_stack.push(data);
    }

    pub fn stack_pop(&mut self) -> StackType {
        match self.the_stack.pop() {
            Some(d) => d,
            None => panic!("run to the hills"),
        }
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
