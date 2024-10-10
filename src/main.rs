use rand::{Fill, Rng, RngCore};
use std::fs::{File, OpenOptions};
use std::io::Write;
use rand::distributions::Alphanumeric;

const CHARACTERS_PER_LINE: usize = 60;
const LINES_PER_FILE: usize = 30;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];
    std::fs::remove_file(file_name).unwrap();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(file_name)
        .unwrap();
    let mut file = std::io::BufWriter::new(file);

    let output_kbytes: usize = args[2].parse().unwrap();
    let output_bytes: usize = output_kbytes * 1024;
    let lines = output_bytes / (CHARACTERS_PER_LINE + 1); // 1 byte per character plus a line break
    let last_line_characters = output_bytes % (CHARACTERS_PER_LINE + 1);
    println!("Generating file with 60 characters per line and {lines} lines and {last_line_characters} characters on last line");

    let mut buffer = vec![0; CHARACTERS_PER_LINE];

    for i in 0..lines {
        fillWithRandom(&mut buffer[0..CHARACTERS_PER_LINE]);
        file.write_all(&buffer).unwrap();
        file.write("\n".as_bytes()).unwrap();
    }
    fillWithRandom(&mut buffer[0..last_line_characters]);
    file.write_all(&buffer[0..last_line_characters]).unwrap();

    file.flush().unwrap();
}

fn fillWithRandom(bytes: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let mut rng = rng.sample_iter(&Alphanumeric);
    for i in 0..bytes.len() {
        bytes[i] = rng.next().unwrap();
    }
}
