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

    let expected_output = vec![String::from("h"), String::from("q")];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
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

    let expected_output = vec![String::from("h"), String::from("q")];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
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

    let expected_output = vec![String::from("h"), String::from("q")];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
}
