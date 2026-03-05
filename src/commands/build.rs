use json::JsonValue;
use std::io::{IsTerminal, Read, stdin};
use std::process::exit;

pub fn key(k: String, v: String) {
    if stdin().is_terminal() {
        println!("{}", to_json(k, v));
        return;
    }

    let mut input: String = String::new();

    if stdin().lock().read_to_string(&mut input).is_err() {
        exit(1)
    }

    if input.is_empty() {
        println!("{}", to_json(k, v));
        return;
    }

    match json::parse(input.as_str()) {
        Ok(mut value) => {
            value[k] = v.into();
            println!("{value}")
        }
        Err(error) => {
            println!("JSON Error: {error}")
        }
    }

    fn to_json(k: String, v: String) -> String {
        let mut data = JsonValue::new_object();

        data[k] = v.into();

        data.to_string()
    }
}
