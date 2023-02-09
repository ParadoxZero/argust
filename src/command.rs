use std::collections::HashMap;

#[derive(Debug)]
pub struct ArgContext {
    pub options: HashMap<String, String>,
    pub switches: Vec<String>,
    pub args: Vec<String>,
}

pub struct ParseToken {
    pub option: String,
    pub option_key: String,
    pub switch: String,
}
