use std::error::Error as E;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Write};

use lz4::{Decoder, EncoderBuilder};
use serde::*;

use crate::abstraction::{LinkScript, Object};

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub objects: Vec<Object>,
    pub scripts: Vec<LinkScript>,
}

impl Collection {
    pub fn new(objects: Vec<Object>, scripts: Vec<LinkScript>) -> Self {
        Collection {
            objects,
            scripts,
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn store_encoded(&self, path: &str) -> std::io::Result<()> {
        let output_file = File::create(path)?;
        let mut encoder = EncoderBuilder::new()
            .level(0)
            .build(output_file)?;
        encoder.write(self.to_json().as_bytes())?;
        let (_, r) = encoder.finish();
        r
    }
    pub fn load(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut decoder = Decoder::new(reader)?;
        let mut buffer = String::new();
        decoder.read_to_string(&mut buffer)?;
        let res = simd_json::serde::from_str(buffer.as_mut_str());
        res.map_err(|e| std::io::Error::new(ErrorKind::Other, e.description()))
    }
}