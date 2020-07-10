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
    StringMap(HashMap<String, String>),
}

pub struct Context<'a> {
    the_stack: Vec<StackType>,
    the_environment: HashMap<String, &'a EnvType>,
    program_counter: usize,
}

impl<'a> Context<'a> {
    pub fn new() -> Context<'a> {
        Context {
            the_stack: Vec::new(),
            the_environment: HashMap::new(),
            program_counter: 0,
        }
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
}

impl<'a> fmt::Debug for Context<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Token")
            .field("the_stack", &self.the_stack)
            .field("program_counter", &self.program_counter)
            .finish()
    }
}
