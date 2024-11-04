use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use flate2::read::GzDecoder;
use std::env;
use std::path::Path;


fn main() {
    if let Err(e) = decompress_file() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
fn decompress_file() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename.gz>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input file provided"));
    }
    let input_path = Path::new(&args[1]);
    let output_path = input_path.with_extension("");

    let input_file = File::open(&input_path)?;
    let buffered_reader = BufReader::new(input_file);

    let mut decoder = GzDecoder::new(buffered_reader);

    let output_file = File::create(&output_path)?;
    let mut buffered_writer = BufWriter::new(output_file);

    io::copy(&mut decoder, &mut buffered_writer)?;

    println!("File decompressed to {}", output_path.display());
    Ok(())
}