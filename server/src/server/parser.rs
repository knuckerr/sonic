pub enum Command {
    Quit,
    Get(String),
    Set(String, Vec<u8>),
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
            if let Some((key, value)) = set_command.split_once(' ') {
                Command::Set(key.trim().to_string(), value.trim().as_bytes().to_vec())
            } else {
                Command::Invalid
            }
        }
        ["del", key, ..] => Command::Del(key.to_string()),
        _ => Command::Invalid,
    }
}
