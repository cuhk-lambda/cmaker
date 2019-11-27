use std::fs::File;
use std::io::Read;

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
    let abs_path = if target_type == STATIC {
        let temp = lines[1].split_whitespace().last().unwrap();
        let mut flag = false;
        let mut ignore = false;
        for i in lines[0].split_whitespace() {
            if i == "-o" {
                flag = true;
                continue;
            }
            if i == "qc" {
                ignore = true;
                continue;
            }
            if ignore {
                ignore = false;
                continue;
            }
            if !flag && (i.ends_with(".o")
                || i.ends_with(".a") || i.ends_with(".so")) && !i.contains(",") {
                dependencies.push(path_without_dot(i));
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
            if !flag && (i.ends_with(".o") || i.ends_with(".a") || i.ends_with(".so")) && !(i.contains(",")) {
                dependencies.push(path_without_dot(i));
            }
        }
        name.expect("unable to get name")
    };
    let name = get_last(abs_path.as_str());
    dependencies.dedup_by(|x, y| x == y);
    Target {
        name,
        abs_path,
        dependencies,
        target_type,
    }
}