use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader, prelude::*};
use std::path::Path;
use std::process::Command;

use path_dedot::*;
use rayon::prelude::*;
use crate::TargetType::{StaticLib, DynamicLib, Executable};

static PATH: &'static str = "/home/schrodinger/CLionProject/template/test/trace.log";
static WORK_DIR: &'static str = "/home/schrodinger/CLionProject/template/test";

#[derive(Debug)]
struct Object {
    abs_path: String,
    name: String,
    defined_symbols: Vec<Symbol>,
    undefined_symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Symbol {
    name: String
}

impl Symbol {
    fn new(name: String) -> Self {
        Symbol { name }
    }
    fn demangled(&self) -> String {
        cpp_demangle::Symbol::new(&self.name[..]).unwrap().to_string()
    }
}

impl Object {
    fn new(abs_path: String) -> Self {
        let name = get_last(abs_path.as_str());
        let (defined_symbols, undefined_symbols) = load_symbol(abs_path.as_str());
        Object {
            abs_path,
            name,
            defined_symbols,
            undefined_symbols,
        }
    }
}

#[derive(Debug)]
struct LinkScript {
    abs_path: String,
    target: Target
}

impl LinkScript {
    fn new(abs_path: String) -> Self {
        let target = parse_target(abs_path.as_str());
        LinkScript{
            abs_path,
            target
        }
    }

}
fn work_dir(abs_path: &str) -> &str {
    return &abs_path[0..abs_path.len() - 9];
}

fn parse_target(abs_path: &str) -> Target {
    let mut buffer = String::new();
    let mut file = File::open(abs_path).expect(format!("unable to open {} when parsing target", abs_path).as_str());
    file.read_to_string(&mut buffer).unwrap();
    let lines : Vec<&str> = buffer.lines().collect();
    let target_type = if lines.len() == 2 {
        StaticLib
    } else if lines[0].contains("-shared") {
        DynamicLib
    } else {
        Executable
    };
    let mut dependencies = Vec::new();
    let abs_path = match &target_type {
        StaticLib => {
            let temp = lines[1].split_whitespace().last().unwrap();
            let mut flag = 0;
            for i in lines[0].split_whitespace() {
                if i == "qc" {
                    flag = 1;
                } else if flag == 1 {
                    flag = 2;
                } else if flag == 2 {
                    dependencies.push(path_without_dot(work_dir(abs_path), i));
                }
            }
            path_without_dot(work_dir(abs_path), temp)
        },
        _ => {
            let mut flag = 0;
            let mut name = None;
            for i in lines[0].split_whitespace() {
                if i == "-o" {
                    flag = 1;
                } else if flag == 1 {
                    name.replace(path_without_dot(work_dir(abs_path), i));
                    flag = 2;
                } else if flag == 2 && target_type == DynamicLib {
                    dependencies.push(path_without_dot(work_dir(abs_path), i));
                } else if flag == 2 && target_type == Executable {
                    flag = 3;
                } else if flag == 3 && target_type == Executable {
                    dependencies.push(path_without_dot(work_dir(abs_path), i));
                }
            }
            name.expect("unable to get name")
        }
    };
    let name = get_last(abs_path.as_str());
    Target{
        name,
        abs_path,
        dependencies,
        target_type
    }
}

#[derive(Debug, Eq, PartialEq)]
enum TargetType {
    StaticLib,
    DynamicLib,
    Executable,
}
#[derive(Debug)]
struct Target {
    name: String,
    abs_path: String,
    dependencies: Vec<String>,
    // will changed later
    target_type: TargetType,
}

fn path_without_dot<'a>(work_dir: &str, rev_path: &str) -> String {
    let w = Path::new(work_dir);
    let r = Path::new(rev_path);
    w.join(r).parse_dot().unwrap().to_str().unwrap().to_string()
}

fn extract_symbol(line: &str) -> String {
    let mut i = 0;
    let bytes = line.as_bytes();
    while bytes[i] != ' ' as u8 {
        i += 1;
    }
    String::from(&line[0..i])
}

fn load_symbol(abs_path: &str) -> (Vec<Symbol>, Vec<Symbol>) {
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

fn get_last(content: &str) -> String {
    let bytes = content.as_bytes();
    let mut i = content.len();
    while bytes[i - 1] != '/' as u8 {
        i -= 1;
    }
    String::from(&content[i..content.len()])
}

fn get_abs(content: &Cow<String>, sep: &str, ending: &str) -> String {
    let mut buffer = Vec::new();
    let mut state = 0;
    let bytes = content.as_bytes();
    let length = bytes.len();
    if !content.starts_with("cd") {
        state = 1;
        buffer.resize(WORK_DIR.len(), 0);
        buffer.copy_from_slice(WORK_DIR.as_bytes());
        buffer.push('/' as u8);
    }
    for i in 3..length {
        if length - i >= 3 && &content[i..i + 3] == " &&" {
            buffer.push('/' as u8);
            state = 1;
        }
        if i > sep.len() && &content[i - sep.len()..i] == sep {
            state = 2;
        }
        if state == 2 && &content[i - ending.len()..i] == ending {
            state = 3;
        }
        if state == 0 || state == 2 {
            buffer.push(bytes[i]);
        }
    }
    String::from_utf8(buffer).unwrap()
}

fn main() -> io::Result<()> {
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let all_data = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let lines: Vec<Cow<String>> = all_data
        .par_iter()
        .filter(|x| { !x.starts_with("[") && x.is_ascii() && !x.contains("due to") })
        .map(|x| Cow::Borrowed(x))
        .collect();

    let object_commands: Vec<Cow<String>> = lines
        .par_iter()
        .filter(|x| x.contains(" -o "))
        .cloned()
        .collect();

    let objects: Vec<Object> = object_commands
        .par_iter()
        .map(|x| Object::new(get_abs(x, "-o ", ".o")))
        .collect();

    let linking_commands: Vec<Cow<String>> = lines
        .par_iter()
        .filter(|x| x.contains("link.txt"))
        .cloned()
        .collect();

    let linking_scripts: Vec<LinkScript> = linking_commands
        .par_iter()
        .map(|x| LinkScript::new(get_abs(x, "cmake_link_script ", "link.txt")))
        .collect();

    for i in linking_scripts {
        println!("{:?}", i);
    }

    for i in objects {
        println!("{:?}", i);
    }
    Ok(())
}
