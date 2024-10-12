use crate::error::RTError;
use anyhow::{bail, Result};
use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug)]
pub struct PpmWriter {
    pub path: Box<Path>,
    pub buffer: Box<[[i32; 3]]>,
}

impl PpmWriter {
    pub fn new(file_path: &str) -> Self {
        Self {
            path: Path::new(file_path).into(),
            buffer: Box::new([]),
        }
    }

    pub fn set_buffer(&mut self, buffer: &[[i32; 3]]) {
        self.buffer = buffer.into()
    }

    pub fn write(&self, image_size: [usize; 2]) -> Result<()> {
        if self.buffer.as_ref().len() == 0 {
            bail!(RTError::EmptyBufferOnWrite(
                stringify!("{:?}", self.path.to_string).into(),
            ));
        }

        let file = Self::create_empty_file(&self.path)?;
        let mut buf_writer = BufWriter::new(file);

        writeln!(buf_writer, "P3")?;
        writeln!(buf_writer, "{} {}", image_size[0], image_size[1])?;
        writeln!(buf_writer, "255")?;
        for rgb in self.buffer.as_ref() {
            writeln!(buf_writer, "{} {} {}", rgb[0], rgb[1], rgb[2])?;
        }

        Ok(())
    }

    fn file_exists(path: &Path) -> bool {
        path.exists()
    }

    fn create_empty_file(path: &Path) -> Result<File> {
        if Self::file_exists(path) {
            Ok(OpenOptions::new().write(true).create(true).open(path)?)
        } else {
            Ok(OpenOptions::new().write(true).create_new(true).open(path)?)
        }
    }
}
