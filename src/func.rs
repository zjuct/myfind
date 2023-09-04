use regex::Regex;
use std::fs;
use std::path::Path;

pub fn help(bin: &String) {
    eprintln!("Usage: {} [-h|--help] [-u|--unique] [-v|--verbose] [-p|--path starting-point...] -e|--expression expression...", bin);
}

pub fn find(stps: &Vec<String>, pats: &Vec<Regex>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

pub fn unique(arr: Vec<String>) -> Vec<String> {
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