use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use rayon::prelude::*;

use crate::abstraction::{LinkScript, Object};
use crate::parsing::Collection;
use crate::utils::get_abs;

pub fn parse(work_dir: &str, path: &str) -> io::Result<Collection> {
    let file = File::open(path)?;
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
        .map(|x| Object::new(get_abs(x, "-o ", ".o", work_dir)))
        .collect();

    let linking_commands: Vec<Cow<String>> = lines
        .par_iter()
        .filter(|x| x.contains("link.txt"))
        .cloned()
        .collect();

    let linking_scripts: Vec<LinkScript> = linking_commands
        .par_iter()
        .map(|x| {
            LinkScript::new(get_abs(x, "cmake_link_script ", "link.txt", work_dir))
        })
        .collect();

    let compile = object_commands.par_iter().map(|x| x.to_string()).collect::<Vec<String>>();
    Ok(Collection::new(objects, linking_scripts, compile))
}