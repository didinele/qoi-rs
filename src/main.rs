mod common;
mod decoder;
mod util;

use decoder::Decoder;

fn main() -> eyre::Result<()> {
    let input_path = std::env::args().nth(1).expect("no input path provided");
    let output_path = std::env::args().nth(2).expect("no output path provided");

    let decoder = Decoder::new(&input_path)?;
    decoder.decode(&output_path)?;

    Ok(())
}
