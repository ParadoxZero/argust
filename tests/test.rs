use argust::*;

use std::{ffi::OsString, str::FromStr};

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
    let expected_output = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    assert!(compare(&command_set.args, &expected_output));

    let expected_output = vec![
        String::from_str("h").unwrap(),
        String::from_str("q").unwrap(),
    ];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
}

#[test]
fn with_string_test() {
    let args: Vec<String> = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("-h").unwrap(),
        String::from_str("-q").unwrap(),
        String::from_str("--Option1").unwrap(),
        String::from_str("--Option2=test").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    let args = args.iter();
    let command_set = parse_args(args, None);
    let expected_output = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    assert!(compare(&command_set.args, &expected_output));

    let expected_output = vec![
        String::from_str("h").unwrap(),
        String::from_str("q").unwrap(),
    ];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
}

#[test]
fn with_os_string_test() {
    let args: Vec<OsString> = vec![
        OsString::from_str("Command1").unwrap(),
        OsString::from_str("-h").unwrap(),
        OsString::from_str("-q").unwrap(),
        OsString::from_str("--Option1").unwrap(),
        OsString::from_str("--Option2=test").unwrap(),
        OsString::from_str("Command2").unwrap(),
    ];
    let command_set = parse_args(args.iter(), None);
    let expected_output = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    assert!(compare(&command_set.args, &expected_output));

    let expected_output = vec![
        String::from_str("h").unwrap(),
        String::from_str("q").unwrap(),
    ];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
}
