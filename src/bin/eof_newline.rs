use std::{
    env,
    fs::{self, ReadDir},
    str,
};

fn main() {
    env::args()
        .skip(1)
        .for_each(|arg| match fs::read_dir(&arg) {
            Ok(iter) => standardize(&arg, iter),
            Err(err) => println!("Error scanning {arg}: {err}"),
        });
}

fn standardize(dir: &str, iter: ReadDir) {
    iter.for_each(|elem| match elem {
        Ok(entry) => {
            if let Some(path) = entry.path().to_str() {
                match entry.metadata() {
                    Ok(meta) => {
                        if !meta.is_dir() {
                            match fs::read(path) {
                                Ok(data) => match str::from_utf8(&data) {
                                    Ok(text) => match fs::write(path, check_eof(text)) {
                                        Ok(()) => {}
                                        Err(err) => println!("Error writing {path}: {err}"),
                                    },
                                    Err(err) => println!("Error reading {path}: {err}"),
                                },
                                Err(err) => println!("Error reading {path}: {err}"),
                            }
                        } else {
                            match fs::read_dir(path) {
                                Ok(iter) => standardize(path, iter),
                                Err(err) => println!("Error scanning {path}: {err}"),
                            }
                        }
                    }
                    Err(err) => println!("Error reading {path}: {err}"),
                }
            } else {
                println!("Error scanning {dir}: found entry with non-Unicode name")
            }
        }
        Err(err) => println!("Error scanning {dir}: {err}"),
    })
}

fn check_eof(slice: &str) -> String {
    let mut text = slice.to_string();
    if !text.ends_with("\n") {
        text.push('\n');
    }
    text
}
