use std::io::Write;

use flate2::{write::GzEncoder, Compression};

use crate::{statements::Statement, serialization::DFSerializable, compile::DFCompile};

pub struct CodeLine {
    body: Vec<Statement>
}
impl DFSerializable for CodeLine {
    fn serialize(&self) -> String {
        format!(
            r#"{{"blocks":[{}]}}"#,
            self.body.iter().map(|x| x.serialize()).collect::<Vec<String>>().join(",")
        )
    }
}
impl DFCompile for CodeLine {
    fn compile(&self) -> String {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(self.serialize().as_bytes()).expect("Error gzipping");
        base64::encode(encoder.finish().unwrap())
    }
}
impl CodeLine {
    pub fn new() -> Self {
        Self { body: vec![] }
    }
    pub fn new_from(body: Vec<Statement> ) -> Self {
        Self { body }
    }
    pub fn name(&self) -> String {
        self.body.get(0).map_or_else(|| String::from("Empty"), |b| b.name())
    }
}