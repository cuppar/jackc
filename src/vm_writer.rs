use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct VmWriter {
    file: File,
}

impl VmWriter {
    pub fn new(filepath: &Path) -> io::Result<Self> {
        let file = OpenOptions::new().write(true).create(true).open(filepath)?;
        Ok(Self { file })
    }

    pub fn writePush(&mut self, segment: &str, index: i32) -> io::Result<()> {
        let buf = format!("push {segment} {index}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writePop(&mut self, segment: &str, index: i32) -> io::Result<()> {
        let buf = format!("pop {segment} {index}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeArithmetic(&mut self, command: &str) -> io::Result<()> {
        let buf = format!("{command}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeLabel(&mut self, label: &str) -> io::Result<()> {
        let buf = format!("label {label}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeGoto(&mut self, label: &str) -> io::Result<()> {
        let buf = format!("goto {label}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeIf(&mut self, label: &str) -> io::Result<()> {
        let buf = format!("if-goto {label}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeCall(&mut self, name: &str, n_args: i32) -> io::Result<()> {
        let buf = format!("call {name} {n_args}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeFunction(&mut self, name: &str, n_vars: i32) -> io::Result<()> {
        let buf = format!("function {name} {n_vars}\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn writeReturn(&mut self) -> io::Result<()> {
        let buf = format!("return\n");
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        // file will auto close
        // do nothing
        Ok(())
    }
}
