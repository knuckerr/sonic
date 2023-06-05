use std::time::Duration;

pub enum Command {
    Quit,
    Get(String),
    Set(String, Vec<u8>, Option<Duration>),
    Del(String),
    EXP(String, Duration),
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
            if let Ok((key, option, value)) = parse_set_command(&set_command) {
                if option > 0 {
                    Command::Set(key, value, Some(Duration::from_secs(option)))
                } else {
                    Command::Set(key, value, None)
                }
            } else {
                Command::Invalid
            }
        }
        ["exp", key, value, ..] => {
            if let Ok(seconds) =  value.parse::<u64>() {
                Command::EXP(key.to_string(), Duration::from_secs(seconds))
            } else {
                Command::Invalid
            }
        }
        ["del", key, ..] => Command::Del(key.to_string()),
        _ => Command::Invalid,
    }
}

pub fn parse_set_command(command_str: &str) -> Result<(String, u64, Vec<u8>), String> {
    let mut option: u64 = 0;
    let mut value_str = Vec::new();

    let mut parts = command_str.split_whitespace().peekable();
    let key = parts.next().ok_or("Invalid command")?.to_string();
    let mut parts = parts.peekable();
    while let Some(&value) = parts.peek() {
        match value {
            "ex" => {
                let cmd = parts.next().ok_or("Invalid command")?;
                let cmd_value = parts.next().ok_or("Invalid command")?;
                if let Ok(time) = cmd_value.parse::<u64>() {
                    option = time;
                } else {
                    value_str.push(cmd);
                    value_str.push(cmd_value);
                }
            }
            _ => {
                let string = parts.next().ok_or("Invalid command")?;
                value_str.push(string);
            }
        }
    }
    if value_str.is_empty() {
        return Err("Invalid command".to_string());
    }
    Ok((key, option, value_str.join(" ").as_bytes().to_vec()))
}

#[test]
pub fn test_set_parse_json() {
    let (_, option, value_str) = parse_set_command("key {\"hello\": \"world\"} ex 12").unwrap();
    assert_eq!(12, option);
    assert_eq!("{\"hello\": \"world\"}".as_bytes(), value_str);
}

#[test]
pub fn test_set_parse_multi_spaces() {
    let (_, option, value_str) =
        parse_set_command("key avavava ex vava ex vavava vava ex 12").unwrap();
    assert_eq!(12, option);
    assert_eq!("avavava ex vava ex vavava vava".as_bytes(), value_str);
}

#[test]
pub fn test_set_parse_multi_ex() {
    let (_, option, value_str) = parse_set_command("key \"avavava ex vavava\" ex 129292").unwrap();
    assert_eq!(129292, option);
    assert_eq!("\"avavava ex vavava\"".as_bytes(), value_str);
}

#[test]
pub fn test_set_parse_alone_ex() {
    let (_, option, value_str) = parse_set_command("key \"ex\"").unwrap();
    assert_eq!(0, option);
    assert_eq!("\"ex\"".as_bytes(), value_str);
}

#[test]
pub fn test_set_parse_quotes_ex() {
    let (_, option, value_str) =
        parse_set_command("key \"heue jsjslaoxnsjex jsjsex sjsjs ex\"").unwrap();
    assert_eq!(0, option);
    assert_eq!(
        "\"heue jsjslaoxnsjex jsjsex sjsjs ex\"".as_bytes(),
        value_str
    );
}
