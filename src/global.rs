use clap::ArgMatches;

use crate::cli::{get_item, get_matches};

lazy_static! {
    static ref MATCH: ArgMatches<'static> = get_matches();
    pub static ref PATH: &'static str = get_item("trace", &*MATCH);
    pub static ref WORK_DIR: &'static str = get_item("work_dir", &*MATCH);
    pub static ref OUTPUT: &'static str = get_item("output", &*MATCH);
}

