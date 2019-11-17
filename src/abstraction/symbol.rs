use std::process::Command;

use rayon::prelude::*;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    name: String
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }
    pub fn demangled(&self) -> String {
        cpp_demangle::Symbol::new(&self.name[..]).unwrap().to_string()
    }
}

fn extract_symbol(line: &str) -> String {
    let mut i = 0;
    let bytes = line.as_bytes();
    while bytes[i] != ' ' as u8 {
        i += 1;
    }
    String::from(&line[0..i])
}

pub fn load_symbol(abs_path: &str) -> (Vec<Symbol>, Vec<Symbol>) {
    let output = Command::new("nm")
        .arg(abs_path)
        .arg("-f")
        .arg("posix")
        .output()
        .expect("failed to execute process");
    let stdout = String::from_utf8(output.stdout).unwrap();
    (stdout.par_lines()
         .filter(|x| x.contains(" T "))
         .map(|x| extract_symbol(x))
         .map(|x| Symbol::new(x))
         .collect(),
     stdout.par_lines()
         .filter(|x| x.contains(" U "))
         .map(|x| extract_symbol(x))
         .map(|x| Symbol::new(x))
         .collect())
}

