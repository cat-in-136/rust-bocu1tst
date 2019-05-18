use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::env;
use rust_bocu1tst::file::{Bocu1Error, encode_file, decode_file};

fn main_encode(filename: &String) -> Result<i8, Bocu1Error> {
    let mut fin = BufReader::new(File::open(filename)?);
    let mut fout = BufWriter::new(File::create("bocu-1.txt")?);

    encode_file(&mut fin, &mut fout)
}

fn main_decode(filename: &String) -> Result<i8, Bocu1Error> {
    let mut fin = BufReader::new(File::open("bocu-1.txt")?);
    let mut fout = BufWriter::new(File::create(filename)?);

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
