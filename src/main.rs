use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // Open input file
    let mut input_file = File::open("input.xml")?;

    // Read message from input file
    let mut message = String::new();
    input_file.read_to_string(&mut message)?;

    // Calculate message length
    let message_length = message.len() as u32;

    // Convert message length to big-endian bytes
    let len_bytes = message_length.to_be_bytes();

    // Open output file
    let mut output_file = File::create("output.xml")?;

    // Write the 4-byte header indicating message length to output file
    output_file.write_all(&len_bytes)?;

    // Write the message itself to output file
    output_file.write_all(message.as_bytes())?;

    Ok(())
}

