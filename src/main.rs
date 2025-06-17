use crate::mochawesome::ParsedMochawesome;
use reqwest::Client;
use std::env;
use std::fs;
use std::process;

mod mochawesome;

enum Command {
    File,
    TextOnly,
    Host,
    Port,
    Database,
    Token,
}

fn parse_argument(arg: &str) -> Command {
    match arg.to_lowercase().as_str() {
        "--file" => Command::File,
        "--text-only" => Command::TextOnly,
        "--host" => Command::Host,
        "--port" => Command::Port,
        "--database" => Command::Database,
        "--token" => Command::Token,
        _ => panic!("Invalid command: {arg}"),
    }
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next(); // skip the first argument as we dont need it

    // data needed for the program
    let mut file_path = String::new();
    let mut text_only = false;
    let mut host = String::new();
    let mut port = 8181;
    let mut database = String::new();
    let mut token = String::new();

    // parse all of the command arguments
    while let Some(arg) = args.next() {
        match parse_argument(&arg) {
            Command::File => file_path = args.next().expect("missing file_path value"),
            Command::TextOnly => text_only = true,
            Command::Host => host = args.next().expect("missing --host value"),
            Command::Port => {
                port = args
                    .next()
                    .expect("missing --port value")
                    .parse()
                    .unwrap_or_else(|err| panic!("Error parsing --port: {err}"))
            }
            Command::Database => database = args.next().expect("missing --database value"),
            Command::Token => token = args.next().expect("missing --token value"),
        }
    }

    // check for mandatory arguments
    if file_path.is_empty() {
        panic!("please provide --file options");
    }
    if !text_only && host.is_empty() {
        if host.is_empty() {
            panic!("Must provide --host if --text-only is NOT provided");
        }
        if database.is_empty() {
            panic!("Must provide --database if --text-only is NOT provided");
        }
        if token.is_empty() {
            panic!("Must provide --token if --text-only is NOT provided");
        }
    }

    // parse from generated mochawesome cypress test report to InfluxDB 3 Protocol Line
    let the_json = fs::read_to_string(file_path).unwrap_or_else(|err| panic!("Error: {err}"));
    let parsed_mochawesome: ParsedMochawesome = serde_json::from_str(&the_json)
        .unwrap_or_else(|err| panic!("Error when parsing file: {err}"));
    let protocol_line = parsed_mochawesome.to_protocol_line();

    // decide what to do after get protocol lines
    if text_only {
        println!("{protocol_line}");
        process::exit(0);
    }

    let url = format!("http://{host}:{port}/api/v3/write_lp?db={database}");
    println!("Call POST to {url}");
    match Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {token}"))
        .body(protocol_line)
        .send()
        .await
    {
        Ok(res) => println!("Data successfully inserted: {res:?}"),
        Err(err) => panic!("Error: {err}"),
    };
}
