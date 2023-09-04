use regex::Regex;
use std::{fs, process, env};
use std::path::Path;
use tracing::info;
use tracing_subscriber;
use std::error::Error;

const ANSI_RED: &str = "\x1b[0;31m";
const ANSI_GREEN: &str = "\x1b[0;32m";
const ANSI_YELLOW : &str = "\x1b[0;33m";
const ANSI_END: &str = "\x1b[0m";

#[tracing::instrument]
fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

//    let span = span!(Level::DEBUG, "span");
//    let _enter = span.enter();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help(&args[0]);
        process::exit(1);
    }
    
    let mut stps: Vec<String> = Vec::new();
    let mut pats: Vec<Regex> = Vec::new();
    let mut sf = false;     // stps flag 
    let mut pf = false;     // pats flag
    let mut uniq = false;
    let mut verbose = false;
    for tok in &args[1..] {
        let tok = tok.trim();
        match tok {
            "-h" | "--help" => {
                help(&args[0]);
                process::exit(1);
            },
            "-u" | "--unique" => {
                uniq = true;
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "-p" | "--path" => {
                sf = true;
                pf = false;
            },
            "-e" | "--expression" => {
                pf = true;
                sf = false;
            }, 
            _ => {
                if tok.starts_with("-") {
                    eprintln!("{}Unrecognized option:{} {}", ANSI_RED, ANSI_END, tok);
                    process::exit(1);
                } else {
                    if sf == true {
                        stps.push(tok.to_string());
                    } else if pf == true {
                        pats.push(match Regex::new(tok) {
                            Ok(re) => re,
                            Err(err) => {
                                eprintln!("{}Invalid regular expression{} {} : {}", ANSI_RED, ANSI_END, tok, err);
                                process::exit(1);
                            }
                        })
                    } else {
                        eprintln!("{}Invalid argument:{} {}", ANSI_RED, ANSI_END, tok);
                        process::exit(1);
                    }
                }
            }
        }
    }

    // If no starting point is specified, '.' is used.
    if stps.len() == 0 {
        stps.push(String::from("."));
    }
    if pats.len() == 0 {
        eprintln!("{}At least one regular expression should be given!{}", ANSI_RED, ANSI_END);
        process::exit(1);
    }

    info!("Starting points of find:");
    for stp in &stps {
        info!("{}", stp);
    }
    info!("Regular expression patterns:");
    for pat in &pats {
        info!("{}", pat.as_str());
    }

    match find(&stps, &pats) {
        Ok(mut matches) => {
            if matches.is_empty() {
                println!("{}No match found.{}", ANSI_RED, ANSI_END);
            } else {
                println!("{}Matches found: {}", ANSI_GREEN, ANSI_END);
                if uniq == true {
                    matches.sort();
                    matches = unique(matches);
                }
                for file in &matches {
                    if verbose == true {
                        println!("{}{}{}", ANSI_YELLOW, file, ANSI_END);
                        let buf: String = match fs::read_to_string(file) {
                            Ok(str) => str,
                            Err(err) => {
                                eprintln!("{}{}{}", ANSI_RED, err, ANSI_END);
                                process::exit(1);
                            }
                        };
                        println!("{}", buf);
                    } else {
                        println!("{}", file);
                    }
                }
            }
        },
        Err(error) => {
            eprintln!("{}Error encountered:{} {}", ANSI_RED, ANSI_END, error);
            process::exit(1);
        }
    }
    return Ok(())
}

fn help(bin: &String) {
    eprintln!("Usage: {} [-h] [-p starting-point...] [-e expression...]", bin);
}

fn find(stps: &Vec<String>, pats: &Vec<Regex>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches: Vec<String> = Vec::new();
    for stp in stps {
        for pat in pats {
            walk_tree(&Path::new(stp), pat, &mut matches)?;
        }
    }
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    matches: &mut Vec<String>
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
//            println!("{}", entry.path().to_string_lossy().to_string());
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()){
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

fn unique(arr: Vec<String>) -> Vec<String> {
    let mut arr_new: Vec<String> = Vec::new();
    let mut idx = 0;
    let sz = arr.len();
    while idx < sz {
        if idx == sz - 1 || &arr[idx] != &arr[idx + 1] {
            arr_new.push(arr[idx].clone());
        }
        idx = idx + 1;
    }
    arr_new
}