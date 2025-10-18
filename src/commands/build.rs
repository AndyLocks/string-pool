use json::JsonValue;
use std::io::{IsTerminal, Read, stdin};
use std::process::exit;

pub fn build(k: String, v: String) {
    if stdin().is_terminal() {
        let mut data = JsonValue::new_object();

        data[k] = v.into();

        println!("{data}");

        return;
    }

    let mut input: String = String::new();

    if stdin().lock().read_to_string(&mut input).is_err() {
        exit(1)
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
}
