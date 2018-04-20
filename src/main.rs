use std::env;
use std::io;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::BufWriter;
use std::io::Write;

fn main_decode(filename: &String) -> Result<i8,io::Error> {
    let fin = BufReader::new(fs::File::open(filename)?);
    let mut fout = BufWriter::new(fs::File::create("bocu-1.txt")?);

    for b in fin.bytes() {
        // TODO convert
        fout.write(&[b?])?;
    }

    Ok(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let exit_val = if (args.len() == 3) && (args[1] == "encode") {
        unimplemented!("encode not implement yet");
        0
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
