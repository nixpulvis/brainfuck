use std::io::Read;
use std::path::Path;
use std::fs::File;
use super::Error;

// TODO: Compress and cache the code, removing everything but code.
//       This will allow running to avoid the overhead of finding
//       instructions and brace matching.
pub struct Program {
    source: String,
}

impl Program {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Program, Error> {
        let mut file = try!(File::open(path));
        let mut source = String::new();
        try!(file.read_to_string(&mut source));
        Ok(Program::from_source(source))
    }

    pub fn from_source<C: Into<String>>(source: C) -> Program {
        Program {
            source: source.into(),
        }
    }

    /// TODO: Is this the right idea?
    pub fn source(&self) -> &str {
        &self.source
    }

    // fn compress(&mut self) {}
    // fn check(&self) {}
    // fn optimize(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program() {
        let program = Program::from_file("fixtures/hello.b");
        assert!(program.is_ok());
    }
}
