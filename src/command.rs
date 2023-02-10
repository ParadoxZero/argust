use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArgContext {
    pub options: HashMap<String, String>,
    pub switches: Vec<String>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParseToken {
    pub option: String,
    pub option_key: String,
    pub switch: String,
}
