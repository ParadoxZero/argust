use std::collections::{HashMap, HashSet};

use crate::utils::new_string;

#[derive(Debug, Clone)]
pub struct ArgContext {
    pub long_params: HashMap<String, Option<String>>,
    pub short_params: HashMap<String, Option<String>>,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ParseTokens {
    pub long_token: String,
    pub option_key: String,
    pub short_token: String,
}

#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub parse_tokens: ParseTokens,
    pub parameterized_long_params: HashSet<String>,
    pub parameterized_short_params: HashSet<String>,
}

impl ParseTokens {
    pub fn new() -> ParseTokens {
        ParseTokens {
            long_token: new_string!("--"),
            option_key: new_string!("="),
            short_token: new_string!("-"),
        }
    }
}

impl ParserConfig {
    pub fn new() -> ParserConfig {
        ParserConfig {
            parse_tokens: ParseTokens::new(),
            parameterized_short_params: HashSet::new(),
            parameterized_long_params: HashSet::new(),
        }
    }

    pub fn get_parse_config(parse_config: Option<ParserConfig>) -> ParserConfig {
        parse_config.or_else(|| Some(ParserConfig::new())).unwrap()
    }
}
