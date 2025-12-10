use crate::commands::unwrap_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};

pub fn get(dir: Option<PathBuf>, key: &str, enable_safe_format: bool) -> std::io::Result<()> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {

        eprintln!("The directory {dir:?} does not exist.");
        exit(1)
    }

    let path = dir.join(key);

    if path.is_file() {
        if io::stdin().is_terminal() {
            std::io::copy(&mut File::open(path)?, &mut std::io::stdout().lock())?;

            return Ok(());
        }

        let json = &mut String::new();

        io::stdin().read_to_string(json)?;

        match json::parse(json) {
            Err(error) => println!("JSON Error: {error}"),
            Ok(value) => {
                let writer = &mut io::stdout().lock();

                if let json::JsonValue::Object(object) = value {
                    BufReader::new(File::open(path)?)
                        .lines()
                        .map(|l| l.unwrap())
                        .for_each(|s| {
                            let mut new_string = s.clone();

                            for (k, v) in object.iter() {
                                new_string = new_string
                                    .replace(format!("${{{k}}}").as_str(), v.as_str().unwrap());

                                if enable_safe_format {
                                    new_string = new_string.replace("\\{", "{")
                                }
                            }

                            new_string.push('\n');
                            writer.write_all(new_string.as_bytes()).unwrap()
                        });
                }
                return Ok(());
            }
        }
    }

    exit(1)
}
