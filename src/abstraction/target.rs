use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub abs_path: String,
    pub dependencies: Vec<String>,
    // will changed later
    pub target_type: u8,
    pub linking_args: Vec<String>,
    pub ranlib_args: Vec<String>,
}

pub static EXEC: u8 = 0;
pub static SHARED: u8 = 1;
pub static STATIC: u8 = 2;
// type
// 0: executable
// 1: shared_lib
// 2: static_lib
