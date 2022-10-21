use std::io::stdin;

pub(crate) trait Input {
    fn read(&self) -> String;
}

pub struct StdInReader;

impl Input for  StdInReader{
    fn read(&self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Reading stdin failed");
        return input;
    }
}
