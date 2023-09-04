use regex::Regex;
use std::{fs, process, env};
use tracing::info;
use tracing_subscriber;
use std::error::Error;

const ANSI_RED: &str = "\x1b[0;31m";
const ANSI_GREEN: &str = "\x1b[0;32m";
const ANSI_YELLOW : &str = "\x1b[0;33m";
const ANSI_END: &str = "\x1b[0m";

mod func;

#[tracing::instrument]
fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        func::help(&args[0]);
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
                func::help(&args[0]);
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

    match func::find(&stps, &pats) {
        Ok(mut matches) => {
            if matches.is_empty() {
                println!("{}No match found.{}", ANSI_RED, ANSI_END);
            } else {
                println!("{}Matches found: {}", ANSI_GREEN, ANSI_END);
                if uniq == true {
                    matches.sort();
                    matches = func::unique(matches);
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
