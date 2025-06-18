use cymochawesome_to_idb3pl;
use cymochawesome_to_idb3pl::Command;
use cymochawesome_to_idb3pl::is_valid_value;
use cymochawesome_to_idb3pl::mochawesome::ParsedMochawesome;
use reqwest::Client;
use std::env;
use std::fs;
use std::process;

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
    let mut table_name = String::new();

    // parse all of the command arguments
    while let Some(arg) = args.next() {
        match cymochawesome_to_idb3pl::parse_argument(&arg) {
            Command::File => file_path = is_valid_value(args.next(), "missing file_path value"),
            Command::TextOnly => text_only = true,
            Command::Host => host = is_valid_value(args.next(), "missing --host value"),
            Command::Port => {
                port = is_valid_value(args.next(), "missing --port value")
                    .parse()
                    .unwrap_or_else(|err| {
                        eprintln!("Error parsing --port: {err}");
                        process::exit(1);
                    })
            }
            Command::Database => database = is_valid_value(args.next(), "missing --database value"),
            Command::Token => token = is_valid_value(args.next(), "missing --token value"),
            Command::Table => table_name = is_valid_value(args.next(), "missing --table value"),
            Command::Invalid(arg) => {
                eprintln!("Invalid command: {arg}");
                process::exit(1);
            }
        }
    }

    // check for mandatory arguments
    if file_path.is_empty() {
        eprintln!("please provide --file options");
        process::exit(1);
    }
    if !text_only {
        if host.is_empty() {
            eprintln!("Must provide --host if --text-only is NOT provided");
        }
        if database.is_empty() {
            eprintln!("Must provide --database if --text-only is NOT provided");
        }
        if token.is_empty() {
            eprintln!("Must provide --token if --text-only is NOT provided");
        }
        process::exit(1);
    }

    // parse from generated mochawesome cypress test report to InfluxDB 3 Protocol Line
    let the_json = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });
    let parsed_mochawesome: ParsedMochawesome =
        serde_json::from_str(&the_json).unwrap_or_else(|err| {
            eprintln!("Error when parsing file: {err}");
            process::exit(1);
        });
    let protocol_line = parsed_mochawesome.to_protocol_line(table_name.as_str());

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
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };
}
