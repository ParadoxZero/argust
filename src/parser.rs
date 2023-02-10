use crate::utils::new_string;
use crate::ArgContext;
use crate::ParseToken;
use std::collections::HashMap;
use std::ffi::OsString;
use std::str::FromStr;

pub trait ToRustString {
    fn to_rstr(&self) -> String;
}

pub fn parse_args<T>(
    args: impl Iterator<Item = T>,
    custom_parse_token: Option<ParseToken>,
) -> ArgContext
where
    T: ToRustString,
{
    let parse_token = get_parse_token(custom_parse_token);

    let mut options: HashMap<String, String> = HashMap::new();
    let mut switches: Vec<String> = Vec::new();
    let mut commands: Vec<String> = Vec::new();

    for arg in args {
        let arg = arg.to_rstr();
        if arg.starts_with(&parse_token.option) {
            let values = arg.replace(&parse_token.option, "");
            let values: (String, String) = split_key_value(values, &parse_token.option_key);
            options.insert(values.0, values.1);
        } else if arg.starts_with(&parse_token.switch) {
            switches.push(arg.replace(&parse_token.switch, ""));
        } else {
            commands.push(arg.to_string());
        }
    }
    return ArgContext {
        options,
        switches,
        args: commands,
    };
}

// Privates

fn split_key_value(values: String, delimiter: &str) -> (String, String) {
    match values.split_once(delimiter) {
        None => (values, String::new()),
        Some(a) => (
            FromStr::from_str(a.0.clone()).expect("Failed to convert to string"),
            FromStr::from_str(a.1.clone()).expect("Failed to convert to string"),
        ),
    }
}

fn get_parse_token(parse_token: Option<ParseToken>) -> ParseToken {
    let parse_token = match parse_token {
        Some(token) => token,
        None => ParseToken {
            option: new_string!("--"),
            option_key: new_string!("="),
            switch: new_string!("-"),
        },
    };
    return parse_token;
}

impl ToRustString for &OsString {
    fn to_rstr(&self) -> String {
        self.to_string_lossy().to_string()
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
