use std::env;
use std::io;
use std::fs;
use std::io::BufReader;
use std::io::Read;
use std::io::BufWriter;
use std::io::Write;

mod bocu1;
use bocu1::Boku1Rx;

fn decode_file(fin: &mut BufReader<fs::File>, fout: &mut BufWriter<fs::File>) -> Result<i8,io::Error> {
    let mut rx = Boku1Rx::new();
    let mut bytes: [u8; 4] = [0; 4];

    for b in fin.bytes() {
        let c = rx.decode_bocu1(b?);

        if c < -1 {
            panic!("error: illegal BOCU-1 sequence at file byte index ");
        }

        if c >= 0 {
            match std::char::from_u32(c as u32) {
                Some(v) => fout.write_fmt(format_args!("{}", v))?,
                None => {
                    panic!("Error");
                }
            };
        }
    }

    Ok(1)
}

fn main_decode(filename: &String) -> Result<i8,io::Error> {
    let mut fin = BufReader::new(fs::File::open("bocu-1.txt")?);
    let mut fout = BufWriter::new(fs::File::create(filename)?);

    decode_file(&mut fin, &mut fout)
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
