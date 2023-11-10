use std::fs::read_to_string;

pub struct Scanner {}

impl Scanner {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run_file(&self, pathname: String) -> Result<(), String> {
        match read_to_string(pathname){
            Err(msg) => return Err(msg.to_string()),
            Ok(source) => Self::run(source)
        }
    }

    pub fn run(source: String) -> Result<(), String> {
        println!("{}", source);
        Ok(())
    }

}
