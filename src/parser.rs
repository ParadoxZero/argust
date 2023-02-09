use crate::utils::new_string;
use crate::CommandSet;
use crate::ParseToken;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse_input(args: Vec<String>, custom_parse_token: Option<ParseToken>) -> CommandSet {
    let parse_token = get_parse_token(custom_parse_token);

    let mut options: HashMap<String, String> = HashMap::new();
    let mut switches: Vec<String> = Vec::new();
    let mut commands: Vec<String> = Vec::new();

    for arg in args {
        if arg.starts_with(&parse_token.option) {
            let values = arg.replace(&parse_token.option, "");
            let values: (String, String) = split_key_value(values, &parse_token.option_key);
            options.insert(values.0, values.1);
        } else if arg.starts_with(&parse_token.switch) {
            switches.push(arg.replace(&parse_token.switch, ""));
        } else {
            commands.push(arg);
        }
    }
    return CommandSet {
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
