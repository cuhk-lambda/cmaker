use std::fs::File;
use std::io::{BufWriter, Write};

use serde::*;

use crate::abstraction::{LinkScript, Object};

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub objects: Vec<Object>,
    pub scripts: Vec<LinkScript>,
    pub compile: Vec<String>
}

impl Collection {
    pub fn new(objects: Vec<Object>, scripts: Vec<LinkScript>, compile: Vec<String>) -> Self {
        Collection {
            objects,
            scripts,
            compile
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn store_encoded(&self, path: &str) -> std::io::Result<()> {
        let output_file = File::create(path)?;
        let mut writer = BufWriter::new(output_file);
        writer.write(self.to_json().as_bytes())?;
        writer.flush()
    }

//    pub fn load(path: &str) -> std::io::Result<Self> {
//        let file = File::open(path)?;
//        let mut reader = BufReader::new(file);;
//        let mut buffer = String::new();
//        reader.read_to_string(&mut buffer)?;
//        let res = simd_json::serde::from_str(buffer.as_mut_str());
//        res.map_err(|e| std::io::Error::new(ErrorKind::Other, e))
//    }
}