//! Bocu1
#![warn(missing_docs)]

use std::fmt;
use std::io;
use std::io::{Read, Write};

use super::*;

/// Bocu1 generic error type.
#[derive(Debug)]
pub enum Bocu1Error {
    /// IO Error
    Io(io::Error),
    /// Bocu1-UTF8 Conversion Error
    Bocu1(&'static str),
}

impl fmt::Display for Bocu1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Bocu1Error::Io(ref err) => err.fmt(f),
            Bocu1Error::Bocu1(desc) => write!(f, "{}", desc),
        }
    }
}

impl std::error::Error for Bocu1Error {
    fn description(&self) -> &str {
        match *self {
            Bocu1Error::Io(ref err) => err.description(),
            Bocu1Error::Bocu1(desc) => desc,
        }
    }
}

impl From<io::Error> for Bocu1Error {
    fn from(err: io::Error) -> Bocu1Error {
        Bocu1Error::Io(err)
    }
}

/// Read UTF-8 file stream `fin`, conver to BOCU-1, write to the stream `fout`.
///
/// # Examples
///
/// ```
/// let mut fin = "あいうえお\n".as_bytes();
/// let mut fout = Vec::<u8>::new();
///
/// let retval = rust_bocu1tst::file::encode_file(&mut fin, &mut fout);
/// assert_eq!(retval.unwrap(), 0);
/// assert_eq!(fout.to_owned(), &[0xfb, 0x11, 0x59, 0x64, 0x66, 0x68, 0x6a, 0x0a]);
/// ```
pub fn encode_file<R: Read, W: Write>(fin: &mut R, fout: &mut W) -> Result<i8, Bocu1Error> {
    let mut tx = Bocu1Tx::new();
    let mut buffer = String::new();

    fin.read_to_string(&mut buffer)?;

    let buffer = buffer;
    for c in buffer.chars() {
        let bytes = tx.encode_bocu1_as_vec(c as i32);
        fout.write(&bytes[..])?;
    }
    Ok(0)
}

/// Read BOCU-1 file stream `fin`, conver to UTF-1, write to the stream `fout`.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
///
/// let input : &[u8] = &[0xfb, 0x11, 0x59, 0x64, 0x66, 0x68, 0x6a, 0x0a];
/// let mut fin= Cursor::new(input);
/// let mut fout = Vec::<u8>::new();
///
/// let retval = rust_bocu1tst::file::decode_file(&mut fin, &mut fout);
/// assert_eq!(retval.unwrap(), 1);
/// assert_eq!(fout.to_owned(), "あいうえお\n".as_bytes());
/// ```
pub fn decode_file<R: Read, W: Write>(fin: &mut R, fout: &mut W) -> Result<i8, Bocu1Error> {
    let mut rx = Bocu1Rx::new();

    for b in fin.bytes() {
        let c = rx.decode_bocu1(b?);

        if c < -1 {
            return Err(Bocu1Error::Bocu1(
                "Illegal BOCU-1 sequence at file byte index",
            ));
        }

        if c >= 0 {
            match std::char::from_u32(c as u32) {
                Some(v) => fout.write_fmt(format_args!("{}", v))?,
                None => return Err(Bocu1Error::Bocu1("Bocu1 convertion error")),
            };
        }
    }

    Ok(1)
}
