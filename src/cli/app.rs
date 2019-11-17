use clap;
use clap::{App, ArgMatches};
use yaml_rust::*;

macro_rules! get_yaml {
    ($yml:expr) => (
        YamlLoader::load_from_str(include_str!($yml)).expect("failed to load YAML file").pop().expect("unable to load yaml")
    );
}

lazy_static! {
    static ref CONFIG : Yaml  = get_yaml!("config.yml");
}

pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::from_yaml(&*CONFIG).get_matches()
}

pub fn get_item(name: &str, matches: &'static ArgMatches<'static>) -> &'static str {
    matches.value_of(name).unwrap()
}