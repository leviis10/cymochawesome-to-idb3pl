use std::process;

pub mod mochawesome;

pub enum Command {
    File,
    TextOnly,
    Host,
    Port,
    Database,
    Token,
    Table,
    Invalid(String),
}

pub fn parse_argument(arg: &str) -> Command {
    match arg.to_lowercase().as_str() {
        "--file" => Command::File,
        "--text-only" => Command::TextOnly,
        "--host" => Command::Host,
        "--port" => Command::Port,
        "--database" => Command::Database,
        "--token" => Command::Token,
        "--table" => Command::Table,
        _ => Command::Invalid(String::from(arg)),
    }
}

pub fn is_valid_value(value: Option<String>, error_message: &str) -> String {
    value.unwrap_or_else(|| {
        eprintln!("{error_message}");
        process::exit(1);
    })
}
