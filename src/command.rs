use std::collections::HashMap;

pub struct Command {
    pub command: String,
    pub parameters: HashMap<String, String>,
}

pub fn parse_command(input: &str) -> Result<Command, String>{
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty input".to_string());
    }

    let name = parts[0].to_string();
    let mut params = HashMap::new();
    for part in parts.iter().skip(1) {
        let key_value: Vec<&str> = part.split('=').collect();
        if key_value.len() == 2 {
            params.insert(key_value[0].to_string(), key_value[1].to_string());
        } else {
            return Err(format!("Invalid parameter format: {}", part));
        }
    }

    Ok(Command { command:name, parameters:params })
}

pub fn parse_command_option(input: Option<&str>) -> Result<Command, String>{
    match input {
        None => Err("Empty command".to_string()),
        Some(input_) => parse_command(input_),
    }
}