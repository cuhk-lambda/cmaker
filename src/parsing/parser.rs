use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::Read;

use rayon::prelude::*;
use twoway::rfind_bytes;

use crate::abstraction::{LinkScript, Object};
use crate::parsing::Collection;
use crate::utils::{get_abs, output_canonicalize};

pub fn parse(work_dir: &str, path: &str) -> io::Result<Collection> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let all_data = buffer.par_lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let lines: Vec<Cow<String>> = all_data
        .par_iter()
        .filter(|x| { !x.starts_with("[") && x.is_ascii() && !x.contains("due to") })
        .map(|x| Cow::Borrowed(x))
        .collect();

    let object_commands: Vec<Cow<String>> = lines
        .par_iter()
        .filter(|x| rfind_bytes(x.as_bytes(), b" -o ").is_some())
        .cloned()
        .collect();

    let objects: Vec<Object> = object_commands
        .iter()
        .map(|x| Object::new(get_abs(x, "-o ", ".o", work_dir)))
        .collect();

    let linking_commands: Vec<Cow<String>> = lines
        .par_iter()
        .filter(|x| x.contains("link.txt"))
        .cloned()
        .collect();

    let linking_scripts: Vec<LinkScript> = linking_commands
        .iter()
        .map(|x| {
            LinkScript::new(get_abs(x, "cmake_link_script ", "link.txt", work_dir))
        })
        .collect();

    let _compile = object_commands.par_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let compile = _compile.iter().map(|x| output_canonicalize(x)).collect();
    Ok(Collection::new(objects, linking_scripts, compile))
}