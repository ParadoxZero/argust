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

    pub fn add_parameter(&mut self, short: char, long: &str) {
        self.parameterized_short_params.insert(short.to_string());
        self.parameterized_long_params.insert(long.to_string());
    }
}

impl ArgContext {
    pub fn contains_short(&self, short: char) -> (bool, Option<String>) {
        return self.contains(Some(short), None);
    }
    pub fn contains_long(&self, long: &str) -> (bool, Option<String>) {
        return self.contains(None, Some(long));
    }

    /// Return is the `ArgContext` contains the given string and it's value
    /// Since long format is the more verbose parameter. If both long and short
    /// params are provided. The method only values the long one.
    pub fn contains(&self, short: Option<char>, long: Option<&str>) -> (bool, Option<String>) {
        if let Some(long) = long {
            let ret = self.check_value(long, &self.long_params);
            if ret.0 {
                return ret;
            }
        }

        if let Some(short) = short {
            return self.check_value(&short.to_string(), &self.short_params);
        }
        return (false, None);
    }

    fn check_value(
        &self,
        key: &str,
        list: &HashMap<String, Option<String>>,
    ) -> (bool, Option<String>) {
        let key = list.get(key);
        match key {
            Some(value) => return (true, value.to_owned()),
            None => return (false, None),
        };
    }
}
