use std::collections::HashMap;
use std::time::Duration;

pub enum Command {
    Quit,
    Get(String),
    Set(String, Vec<u8>, Option<Duration>),
    Del(String),
    Invalid,
}

pub fn parse_command(command: &str) -> Command {
    let command_lower = command.to_lowercase();
    let parts: Vec<&str> = command_lower.split_whitespace().collect();
    match parts.as_slice() {
        ["quit", ..] => Command::Quit,
        ["get", key, ..] => Command::Get(key.to_string()),
        ["set", key_value @ ..] => {
            let set_command = key_value.join(" ");
            let (key, option, value) = parse_set_command(&set_command);
            if option > 0 {
                Command::Set(key.to_string(), value, Some(Duration::from_secs(option)))
            } else {
                Command::Set(key.to_string(), value, None)
            }
        }
        ["del", key, ..] => Command::Del(key.to_string()),
        _ => Command::Invalid,
    }
}

pub fn parse_set_command(command_str: &str) -> (String, u64, Vec<u8>) {
    let mut option: u64 = 0;
    let mut value_str = Vec::new();

    let mut parts = command_str.split_whitespace().peekable();
    let key = parts.next().unwrap().to_string();
    let mut parts = parts.peekable();
    while let Some(&value) = parts.peek() {
        match value {
            "ex" => {
                let cmd = parts.next();
                let cmd_value = parts.next();
                if let Some(exp) = cmd_value.map(|s| s.parse::<u64>()) {
                    if let Ok(time) = exp {
                        option = time;
                    } else {
                        value_str.push(cmd.unwrap());
                        value_str.push(cmd_value.unwrap());
                    }
                }
            }
            _ => {
                let string = parts.next().unwrap();
                value_str.push(string);
            }
        }
    }
    (key, option, value_str.join(" ").as_bytes().to_vec())
}

#[test]
pub fn set() {
    let (_, option, value_str) = parse_set_command("set {\"hello\": \"world\"} ex 12");
    assert_eq!(12, option);
    assert_eq!("{\"hello\": \"world\"}".as_bytes(), value_str);
}

#[test]
pub fn set3() {
    let (_, option, value_str) = parse_set_command("set avavava ex vava ex vavava vava ex 12");
    assert_eq!(12, option);
    assert_eq!("avavava ex vava ex vavava vava".as_bytes(), value_str);
}

#[test]
pub fn set2() {
    let (_, option, value_str) = parse_set_command("set \"avavava ex vavava\" ex 129292");
    assert_eq!(129292, option);
    assert_eq!("\"avavava ex vavava\"".as_bytes(), value_str);
}

#[test]
pub fn set4() {
    let (_, option, value_str) = parse_set_command("set \"ex\"");
    assert_eq!(0, option);
    assert_eq!("\"ex\"".as_bytes(), value_str);
}

#[test]
pub fn set5() {
    let (_, option, value_str) = parse_set_command("set \"heue jsjslaoxnsjex jsjsex sjsjs ex\"");
    assert_eq!(0, option);
    assert_eq!(
        "\"heue jsjslaoxnsjex jsjsex sjsjs ex\"".as_bytes(),
        value_str
    );
}
