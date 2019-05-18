use std::env;
use std::io;
use std::fs;
use std::fmt;
use std::io::BufReader;
use std::io::Read;
use std::io::BufWriter;
use std::io::Write;

use rust_bocu1tst::*;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Bocu1(&'static str),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => err.fmt(f),
            CliError::Bocu1(desc) => write!(f, "{}", desc),
        }
    }
}

impl std::error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Bocu1(desc) => desc,
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

fn encode_file<R: Read, W: Write>(fin: &mut R, fout: &mut W) -> Result<i8, CliError> {
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

fn main_encode(filename: &String) -> Result<i8, CliError> {
    let mut fin = BufReader::new(fs::File::open(filename)?);
    let mut fout = BufWriter::new(fs::File::create("bocu-1.txt")?);

    encode_file(&mut fin, &mut fout)
}

fn decode_file<R: Read, W: Write>(fin: &mut R, fout: &mut W) -> Result<i8, CliError> {
    let mut rx = Bocu1Rx::new();

    for b in fin.bytes() {
        let c = rx.decode_bocu1(b?);

        if c < -1 {
            return Err(CliError::Bocu1(
                "Illegal BOCU-1 sequence at file byte index",
            ));
        }

        if c >= 0 {
            match std::char::from_u32(c as u32) {
                Some(v) => fout.write_fmt(format_args!("{}", v))?,
                None => return Err(CliError::Bocu1("Bocu1 convertion error")),
            };
        }
    }

    Ok(1)
}

fn main_decode(filename: &String) -> Result<i8, CliError> {
    let mut fin = BufReader::new(fs::File::open("bocu-1.txt")?);
    let mut fout = BufWriter::new(fs::File::create(filename)?);

    decode_file(&mut fin, &mut fout)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let exit_val = if (args.len() == 3) && (args[1] == "encode") {
        match main_encode(&args[2]) {
            Ok(_) => 0,
            Err(v) => {
                eprintln!("Error: {:?}", v);
                1
            }
        }
    } else if (args.len() == 3) && (args[1] == "decode") {
        match main_decode(&args[2]) {
            Ok(_) => 0,
            Err(v) => {
                eprintln!("Error: {:?}", v);
                1
            }
        }
    } else {
        eprintln!("usage:");
        eprintln!("{} encode <filename>", args[0]);
        eprintln!("  -> read UTF-8 <filename>, convert to BOCU-1, write to bocu-1.txt\n");
        eprintln!("{} decode <filename>", args[0]);
        eprintln!("  -> read read BOCU-1 file bocu-1.txt, convert to UTF-8, write to <filename>\n");
        1
    };

    std::process::exit(exit_val);
}
