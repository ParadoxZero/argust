use argust::*;

use std::ffi::OsString;

fn compare(lhs: &Vec<String>, rhs: &Vec<String>) -> bool {
    println!("{:?} || {:?}", lhs, rhs);
    if lhs.len() != rhs.len() {
        return false;
    }
    for i in 0..lhs.len() {
        if lhs[i] != rhs[i] {
            println!("{:?},{:?}", lhs[i], rhs[i]);
            return false;
        }
    }
    return true;
}

#[test]
fn basic_test() {
    let args: Vec<&str> = vec![
        "Command1",
        "-h",
        "-q",
        "--Option1",
        "--Option2=test",
        "Command2",
    ];
    let command_set = parse_args(args.iter(), None);
    let expected_output = vec![String::from("Command1"), String::from("Command2")];
    assert!(compare(&command_set.args, &expected_output));

    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(command_set.short_params["q"], None);
    assert_eq!(command_set.long_params["Option1"], None);
    assert_eq!(command_set.long_params["Option2"], Some("test".to_string()));
}

#[test]
fn with_string_test() {
    let args: Vec<String> = vec![
        String::from("Command1"),
        String::from("-h"),
        String::from("-q"),
        String::from("--Option1"),
        String::from("--Option2=test"),
        String::from("Command2"),
    ];
    let args = args.iter();
    let command_set = parse_args(args, None);
    let expected_output = vec![String::from("Command1"), String::from("Command2")];
    assert!(compare(&command_set.args, &expected_output));

    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(command_set.short_params["q"], None);
    assert_eq!(command_set.long_params["Option1"], None);
    assert_eq!(command_set.long_params["Option2"], Some("test".to_string()));
}

#[test]
fn with_os_string_test() {
    let args: Vec<OsString> = vec![
        OsString::from("Command1"),
        OsString::from("-h"),
        OsString::from("-q"),
        OsString::from("--Option1"),
        OsString::from("--Option2=test"),
        OsString::from("Command2"),
    ];
    let command_set = parse_args(args.iter(), None);
    let expected_output = vec![String::from("Command1"), String::from("Command2")];
    assert!(compare(&command_set.args, &expected_output));

    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(command_set.short_params["q"], None);
    assert_eq!(command_set.long_params["Option1"], None);
    assert_eq!(command_set.long_params["Option2"], Some("test".to_string()));
}

#[test]
fn parametrized_test() {
    let args: Vec<&str> = vec![
        "Command1",
        "-h",
        "param1",
        "-q",
        "--Option1",
        "--Option2",
        "Command2",
    ];
    let mut parse_config = ParserConfig::new();
    parse_config.parse_tokens = ParseTokens {
        long_token: String::from("--"),
        long_seperator: String::from(" "),
        short_token: String::from("-"),
    };
    let command_set = parse_args(args.iter(), Some(parse_config.clone()));
    let expected_output = vec![
        String::from("Command1"),
        String::from("param1"),
        String::from("Command2"),
    ];
    assert!(compare(&command_set.args, &expected_output));

    assert_eq!(command_set.short_params.len(), 2);
    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(command_set.short_params["q"], None);

    assert_eq!(command_set.long_params.len(), 2);
    assert_eq!(command_set.long_params["Option1"], None);
    assert_eq!(command_set.long_params["Option2"], None);

    parse_config
        .parameterized_option_list
        .insert(String::from("Option2"));

    let command_set = parse_args(args.iter(), Some(parse_config.clone()));
    assert_eq!(command_set.short_params.len(), 2);
    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(command_set.short_params["q"], None);

    assert_eq!(command_set.long_params.len(), 2);
    assert_eq!(command_set.long_params["Option1"], None);
    assert_eq!(
        command_set.long_params["Option2"],
        Some(String::from("Command2"))
    );

    parse_config
        .parameterized_switch_list
        .insert(String::from("q"));

    let command_set = parse_args(args.iter(), Some(parse_config.clone()));

    assert_eq!(command_set.short_params.len(), 2);
    assert_eq!(command_set.short_params["h"], None);
    assert_eq!(
        command_set.short_params["q"],
        Some(String::from("--Option1"))
    );

    assert_eq!(command_set.long_params.len(), 1);
    assert_eq!(
        command_set.long_params["Option2"],
        Some(String::from("Command2"))
    );
}
