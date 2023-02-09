use cmd_rust::*;

use std::str::FromStr;

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
    let args: Vec<String> = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("-h").unwrap(),
        String::from_str("-q").unwrap(),
        String::from_str("--Option1").unwrap(),
        String::from_str("--Option2=test").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    let command_set = parse_input(args, None);
    let expected_output = vec![
        String::from_str("Command1").unwrap(),
        String::from_str("Command2").unwrap(),
    ];
    assert!(compare(&command_set.commands, &expected_output));

    let expected_output = vec![
        String::from_str("h").unwrap(),
        String::from_str("q").unwrap(),
    ];
    assert!(compare(&command_set.switches, &expected_output));
    assert_eq!(command_set.options["Option1"], "");
    assert_eq!(command_set.options["Option2"], "test");
}
