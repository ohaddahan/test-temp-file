//! This crate allows creation and automatic deletion (based on Drop trait) of files.
//! This is aimed mostly for testing purposes, for example when testing a parser you probably
//! want to read/write file and validate their content

use rand::Rng;
use std::io::Read;
use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::Seek;

#[derive(Debug)]
pub struct TestTempFile {
    filename: String,
    random_number: i32,
    final_filename: String,
    file: std::fs::File
}

impl Drop for TestTempFile {
    fn drop(&mut self) {
        self.delete_file();
    }
}

impl Write for TestTempFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.file.write_all(buf)
    }
}

impl Read for TestTempFile {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for TestTempFile {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> { self.file.seek(pos) }
}

impl TestTempFile {
    ///
    ///  /// # Arguments
    ///
    /// * `filename` - A String containing the file name
    ///
    /// # Examples
    /// ```
    /// use test_temp_file::TestTempFile;
    /// let mut t = TestTempFile::new(String::from("file_name.txt"));
    /// ```
    pub fn new(filename: String) -> TestTempFile {
        TestTempFile::gen_random_name(filename)
    }

    fn gen_random_name(filename: String) -> TestTempFile {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0, i32::max_value());
        let final_filename = format!("_{}_{}", random_number, filename);
        let file = OpenOptions::new().
            create(true).
            write(true).
            read(true).
            open(final_filename.clone()).unwrap();
        TestTempFile {
            filename,
            random_number,
            final_filename,
            file
        }
    }

    fn delete_file(&mut self) {
        let final_filename = self.final_filename.as_str();
        if Path::new(final_filename).exists() {
            std::fs::remove_file(final_filename);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    use std::str;

    static FILE_NAME: &'static str = "test_file.txt";

    #[test]
    fn test_constructor() {
        let t = TestTempFile::new(String::from(FILE_NAME));
        assert_eq!(FILE_NAME, t.filename);
    }

    #[test]
    fn test_write() {
        let mut t = TestTempFile::new(String::from(FILE_NAME));
        match t.write_all(b"some bytes") {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, e)
        }
    }

    #[test]
    fn test_write_and_read() {
        let mut t = TestTempFile::new(String::from(FILE_NAME));
        let mut buffer = [0; 10];
        let msg = b"some bytes";
        match t.write_all(msg) {
            Ok(_) => assert!(true),
            Err(e) => assert!(false, e)
        }

        // Need to rewind pointer inside file, since after the write we're pointing to the end

        t.seek(SeekFrom::Start(0));

        match t.read(&mut buffer[..]) {
            Ok(n) => {
                assert_eq!(msg,
                           &buffer[..n],
                           "Left:{:#?}\nRight:{:#?}\n{:#?}",
                           msg, &buffer[..n], t)
            },
            Err(e) => assert!(false, format!("Error:{}\n{:#?}", e.to_string(), t))
        }
    }
}