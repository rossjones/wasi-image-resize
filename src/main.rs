use std::error::Error;
use std::process;

use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;

mod scale; 
use scale::Scale;

fn main() {
    let inputf = std::env::args().nth(1).expect("no inputfile given");
    let outf = std::env::args().nth(2).expect("no outputfile given");
    let mut scale = std::env::args().nth(3).expect("no scale given");

    match scale_file(inputf, outf, &mut scale) {
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
        _ => (),
    }
}

// Given the input file, output file and a scaling factor - load the input file,
// scale it by the specified amount and write to the output file.
fn scale_file(inputf: String, outf: String, scale_str: &mut String) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(inputf)?.decode()?;

    let scale = Scale::try_from(scale_str)?;
    let (new_w, new_h) = scale.scale_values(img.width(), img.height())?;
    
    let buf = resize(&img, new_w, new_h, FilterType::Lanczos3);
    buf.save(outf)?;
    
    Ok(())
}

