use crate::ArgContext;
use crate::{ParseTokens, ParserConfig};
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsString;
use std::str::FromStr;

pub trait ToRustString {
    fn to_rstr(&self) -> String;
}

pub fn parse_args<T>(
    arg_list: impl Iterator<Item = T>,
    parser_config: Option<ParserConfig>,
) -> ArgContext
where
    T: ToRustString,
{
    let parser_config: ParserConfig = ParserConfig::get_parse_config(parser_config);
    let parse_tokens = &parser_config.parse_tokens;

    let mut options: HashMap<String, Option<String>> = HashMap::new();
    let mut switches: HashMap<String, Option<String>> = HashMap::new();
    let mut args: Vec<String> = Vec::new();

    let arg_list: Vec<String> = arg_list.map(|a| a.to_rstr()).collect();
    let mut i = 0;
    while i < arg_list.len() {
        let arg = &arg_list[i];
        if arg.starts_with(&parse_tokens.long_token) {
            let (idx, key, value) = process_options(
                i,
                &arg_list,
                &parser_config.parameterized_option_list,
                parse_tokens,
            );
            i = idx;
            options.insert(key, value);
        } else if arg.starts_with(&parse_tokens.short_token) {
            let switch = arg.replace(&parse_tokens.short_token, "");
            let (idx, value) = process_parameterized_args(
                &switch,
                i,
                &arg_list,
                &parser_config.parameterized_switch_list,
            );
            i = idx;
            switches.insert(switch, value);
        } else {
            args.push(arg.clone());
        }
        i = i + 1;
    }
    return ArgContext {
        long_params: options,
        short_params: switches,
        args,
    };
}

// Privates

fn split_key_value(values: String, delimiter: &str) -> (String, Option<String>) {
    match values.split_once(delimiter) {
        None => (values, None),
        Some(a) => (
            FromStr::from_str(a.0).expect("Failed to convert to string"),
            Some(FromStr::from_str(a.1).expect("Failed to convert to string")),
        ),
    }
}

fn process_parameterized_args<'a>(
    key: &str,
    idx: usize,
    arg_list: &'a [String],
    parameterized_list: &'a HashSet<String>,
) -> (usize, Option<String>) {
    if parameterized_list.contains(key) {
        let idx = idx + 1;
        return (idx, arg_list.get(idx).map(|v| v.clone()));
    } else {
        return (idx, None);
    }
}

fn process_options(
    idx: usize,
    arg_list: &[String],
    parameterized_list: &HashSet<String>,
    parser_tokens: &ParseTokens,
) -> (usize, String, Option<String>) {
    let key = arg_list[idx].replace(&parser_tokens.long_token, "");
    if &parser_tokens.long_seperator == " " {
        let (idx, value) = process_parameterized_args(&key, idx, &arg_list, &parameterized_list);
        return (idx, key, value);
    } else {
        let (key, value) = split_key_value(key, &parser_tokens.long_seperator);
        return (idx, key, value);
    }
}

impl ToRustString for &OsString {
    fn to_rstr(&self) -> String {
        self.clone()
            .clone()
            .into_string()
            .expect("Invalid unicode in input")
    }
}

impl ToRustString for &String {
    fn to_rstr(&self) -> String {
        self.to_string()
    }
}

impl ToRustString for &&str {
    fn to_rstr(&self) -> String {
        self.to_string()
    }
}
