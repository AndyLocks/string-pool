use crate::commands::unwrap_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{fs, io};

pub fn get(dir: Option<PathBuf>, key: &str, enable_safe_format: bool) -> std::io::Result<()> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                if name == key {
                    if io::stdin().is_terminal() {
                        std::io::copy(&mut File::open(path)?, &mut std::io::stdout().lock())?;

                        return Ok(());
                    }

                    let json = &mut String::new();

                    io::stdin().read_to_string(json)?;

                    match json::parse(json) {
                        Ok(value) => {
                            let writer = &mut io::stdout().lock();

                            if let json::JsonValue::Object(object) = value {
                                BufReader::new(File::open(path)?)
                                    .lines()
                                    .map(|l| l.unwrap())
                                    .for_each(|s| {
                                        let mut new_string = s.clone();

                                        for (k, v) in object.iter() {
                                            new_string = new_string.replace(
                                                format!("${{{k}}}").as_str(),
                                                v.as_str().unwrap(),
                                            );

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
                        Err(error) => println!("JSON Error: {error}"),
                    }
                }
            }
        }
    }

    exit(1)
}
