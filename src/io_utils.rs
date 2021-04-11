use std::io;
use std::io::Write;

const BUF_SIZE: usize = 1024;

pub struct SkipNewlinesReader<'a, R: 'a + io::Read> {
    inner: &'a mut R,
    buf: [u8; BUF_SIZE],
}

impl<'a, R: io::Read> SkipNewlinesReader<'a, R> {
    pub fn new(inner: &'a mut R) -> Self {
        SkipNewlinesReader {
            inner,
            buf: [0; BUF_SIZE],
        }
    }
}

impl<'a, R: io::Read> io::Read for SkipNewlinesReader<'a, R> {
    fn read(&mut self, mut out_buf: &mut [u8]) -> io::Result<usize> {
        if out_buf.len() == 0 {
            return Ok(0);
        }
        // to avoid complex shenanigans, we will not read more than can fit into our output
        let max_len = BUF_SIZE.min(out_buf.len());
        let bytes_read = self.inner.read(&mut self.buf[0..max_len])?;
        if bytes_read == 0 {
            return Ok(0);
        }

        // take a slice of the whole buffer, then iterate through it searching for a newline
        // write that part out, take a new slice with the remainder
        // and loop until the remainder slice is zero bytes
        let mut bytes_written: usize = 0;
        let mut to_write: &[u8] = &self.buf[0..bytes_read];
        'outer: while to_write.len() > 0 {
            for i in 0..to_write.len() {
                if to_write[i] == 0x0D || to_write[i] == 0x0A {
                    bytes_written += out_buf.write(&to_write[0..i])?;
                    to_write = &to_write[i + 1..];
                    continue 'outer;
                }
            }
            // if we got here, there was no newline left in our buffer
            bytes_written += out_buf.write(to_write)?;
            break;
        }
        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    const NO_NEWLINE: &str = "there is no newline here";
    #[test]
    fn test_no_newline() {
        let mut buf: [u8; NO_NEWLINE.len()] = [0; NO_NEWLINE.len()];
        let src = NO_NEWLINE.to_string();
        let mut inner = &mut src.as_bytes();
        let mut reader = SkipNewlinesReader::new(&mut inner);
        match reader.read(&mut buf) {
            Ok(_bytes_read) => assert_eq!(src.as_bytes(), buf),
            Err(_e) => assert!(false),
        };
    }

    const MIDDLE_NEWLINE: &str = "there is a\nnewline in the middle";
    const MIDDLE_NEWLINE_SKIPPED: &str = "there is anewline in the middle";
    #[test]
    fn test_middle_newline() {
        // the output buffer has to fit the whole string (including the newline!) for a single read
        // call to read all data
        let mut buf: [u8; MIDDLE_NEWLINE.len()] = [0; MIDDLE_NEWLINE.len()];
        let src = MIDDLE_NEWLINE.to_string();
        let mut inner = &mut src.as_bytes();
        let mut reader = SkipNewlinesReader::new(&mut inner);
        match reader.read(&mut buf) {
            Ok(bytes_read) => {
                assert_eq!(bytes_read, MIDDLE_NEWLINE_SKIPPED.len());
                assert_eq!(
                    &buf[0..MIDDLE_NEWLINE_SKIPPED.len()],
                    MIDDLE_NEWLINE_SKIPPED.as_bytes()
                );
            }
            Err(_e) => assert!(false),
        };
    }

    const TRAILING_NEWLINE: &str = "there is a newline at the end\n";
    const TRAILING_NEWLINE_SKIPPED: &str = "there is a newline at the end";
    #[test]
    fn test_trailing_newline() {
        let mut buf: [u8; TRAILING_NEWLINE.len()] = [0; TRAILING_NEWLINE.len()];
        let src = TRAILING_NEWLINE.to_string();
        let mut inner = &mut src.as_bytes();
        let mut reader = SkipNewlinesReader::new(&mut inner);
        match reader.read(&mut buf) {
            Ok(bytes_read) => {
                assert_eq!(bytes_read, TRAILING_NEWLINE_SKIPPED.len());
                assert_eq!(
                    &buf[0..TRAILING_NEWLINE_SKIPPED.len()],
                    TRAILING_NEWLINE_SKIPPED.as_bytes()
                );
            }
            Err(_e) => assert!(false),
        };
    }
}
