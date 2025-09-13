use serde_yaml;
use std::{fs, io};

mod types;

const PIPELINE: &str = "pipeline.yaml";

fn main() -> Result<(), io::Error> {
    let definition = fs::File::open(PIPELINE)?;

    let pipeline: types::Pipeline = match serde_yaml::from_reader(definition) {
        Ok(v) => v,
        Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
    };

    pipeline.execute()
}
