use std::fs::File;
use std::io::Read;

use rayon::prelude::*;
use serde::*;

use crate::abstraction::{EXEC, SHARED, STATIC, Target};
use crate::utils::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkScript {
    abs_path: String,
    target: Target,
}

impl LinkScript {
    pub fn new(abs_path: String) -> Self {
        let target = parse_target(abs_path.as_str());
        LinkScript {
            abs_path,
            target,
        }
    }
}

fn parse_target(abs_path: &str) -> Target {
    let mut buffer = String::new();
    let mut file = File::open(abs_path).expect(format!("unable to open {} when parsing target", abs_path).as_str());
    file.read_to_string(&mut buffer).unwrap();
    let lines: Vec<&str> = buffer.lines().collect();
    let target_type = if lines.len() == 2 {
        STATIC
    } else if lines[0].contains("-shared") {
        SHARED
    } else {
        EXEC
    };
    let mut dependencies = Vec::new();
    let mut linking_args = Vec::new();
    let mut ranlib_args = Vec::new();
    let abs_path = if target_type == STATIC {
        let temp = lines[1].split_whitespace().last().unwrap();
        let mut flag = false;
        let mut ignore = false;
        let mut count = 0;
        let mut plugin = false;
        let mut plugin2 = false;
        for i in lines[1].split_ascii_whitespace() {
            if plugin2 {
                plugin2 = false;
                ranlib_args.push(String::from(i));
                continue;
            }
            if i.starts_with("-") {
                ranlib_args.push(String::from(i));
                if i == "--plugin" {
                    plugin2 = true;
                }
            }
        }
        for i in lines[0].split_whitespace() {
            count += 1;
            if i == "-o" {
                flag = true;
                continue;
            }
            if plugin {
                plugin = false;
                linking_args.push(String::from(i));
                continue;
            }
            if count == 2 {
                linking_args.push(String::from(i));
                ignore = true;
                continue;
            }
            if ignore {
                ignore = false;
                continue;
            }
            if !flag && (i.ends_with(".o")
                || i.ends_with(".a") || i.ends_with(".so")) && memchr::memchr(b',', i.as_bytes()).is_none() {
                dependencies.push(path_without_dot(i));
            }
            if !flag && i.starts_with("-") {
                if i == "--plugin" {
                    plugin = true;
                }
                linking_args.push(String::from(i));
            }
            flag = false;
        }
        path_without_dot(temp)
    } else {
        let mut flag = false;
        let mut name = None;
        for i in lines[0].split_whitespace() {
            if i == "-o" {
                flag = true;
                continue;
            } else if flag {
                name.replace(path_without_dot(i));
                flag = false;
                continue;
            }
            if !flag && (i.ends_with(".o") || i.ends_with(".a") || i.ends_with(".so")) &&
                memchr::memchr(b',', i.as_bytes()).is_none() {
                dependencies.push(path_without_dot(i));
            }
            if !flag && i.starts_with("-") {
                linking_args.push(String::from(i));
            }
        }
        name.expect("unable to get name")
    };
    let name = get_last(abs_path.as_str());
    dependencies.sort_unstable();
    dependencies.dedup_by(|x, y| x == y);
    let dependencies = dependencies.into_par_iter().filter(|x| x != &abs_path).collect();
    Target {
        name,
        abs_path,
        dependencies,
        target_type,
        linking_args,
        ranlib_args,
    }
}