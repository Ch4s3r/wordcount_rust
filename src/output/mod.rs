use std::io::{stdout, Write};

pub(crate) trait Output {
    fn write(&self, output: &str);
}

pub struct StdOutWriter;

impl Output for  StdOutWriter{
    fn write(&self, output: &str) {
        stdout().write_all(output.as_bytes().as_ref()).expect("Failed to write output");
        stdout().flush().expect("Failed to flush");
    }
}
