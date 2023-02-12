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
    pub long_seperator: String,
    pub short_token: String,
}

#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub parse_tokens: ParseTokens,
    pub parameterized_switch_list: HashSet<String>,
    pub parameterized_option_list: HashSet<String>,
}

impl ParseTokens {
    pub fn new() -> ParseTokens {
        ParseTokens {
            long_token: new_string!("--"),
            long_seperator: new_string!("="),
            short_token: new_string!("-"),
        }
    }
}

impl ParserConfig {
    pub fn new() -> ParserConfig {
        ParserConfig {
            parse_tokens: ParseTokens::new(),
            parameterized_option_list: HashSet::new(),
            parameterized_switch_list: HashSet::new(),
        }
    }

    pub fn get_parse_config(parse_config: Option<ParserConfig>) -> ParserConfig {
        parse_config.or_else(|| Some(ParserConfig::new())).unwrap()
    }
}
