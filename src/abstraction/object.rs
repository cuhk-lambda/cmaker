use serde::*;

use crate::abstraction::load_symbol;
use crate::utils::get_last;

use super::Symbol;

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    abs_path: String,
    name: String,
    defined_symbols: Vec<Symbol>,
    undefined_symbols: Vec<Symbol>,
}

impl Object {
    pub fn new(abs_path: String) -> Self {
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