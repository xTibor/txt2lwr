extern crate byteorder;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod data;

use byteorder::{LittleEndian, WriteBytesExt};
use std::fs::File;
use std::io::{Read, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "txt2lwr", about = "Text to LogoWriter 2.0 converter")]
struct Opt {
    #[structopt(short = "i", long = "input", help = "Input file")]
    input: String,

    #[structopt(short = "o", long = "output", help = "Output file")]
    output: String,
}

fn ascii_to_lwr(s: &str) -> Vec<u8> {
    let mut result = Vec::new();

    for c in s.chars() {
        if c == '\n' {
            result.push(0x0D); // Carrige return
            result.push(0x81); // Color
        } else {
            result.push(c as u8);
            result.push(0x01); // Color
        }
    }

    result
}

// cargo run -- --input "test/tone.txt" --output ~/msdos/logo/STARTUP.LWR && dosbox

fn main() {
    let opt = Opt::from_args();

    let mut output = File::create(opt.output).unwrap();

    let source_segment = {
        let mut file = File::open(opt.input).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        ascii_to_lwr(&buffer)
    };

    // Segment lengths
    output.write_u16::<LittleEndian>(0x0000).unwrap();
    output.write_u16::<LittleEndian>(source_segment.len() as u16).unwrap();
    output.write_u16::<LittleEndian>(data::LWR_UNKNOWN_SEGMENT.len() as u16).unwrap();
    for _ in 0..5 {
        output.write_u16::<LittleEndian>(0x0000).unwrap();
    }

    // Header
    output.write_all(data::LWR_PAGE_HEADER).unwrap();

    // Segments
    output.write_all(&source_segment).unwrap();
    output.write_all(data::LWR_UNKNOWN_SEGMENT).unwrap();
    output.write_all(data::LWR_MCGA_GRAPHICS_SEGMENT).unwrap();
}
