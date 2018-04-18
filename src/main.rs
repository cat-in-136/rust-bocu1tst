use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let exit_val = if (args.len() == 3) && (args[1] == "encode") {
        unimplemented!("encode not implement yet");
        0
    } else if (args.len() == 3) && (args[1] == "decode") {
        unimplemented!("decode not implement yet");
        0
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
