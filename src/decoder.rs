use crate::common::file::QoiFile;
use crate::common::header::QoiChannel;
use crate::common::op::QoiOp;
use crate::common::pixel::Pixel;
use std::fs::File;
use std::io::BufWriter;

// This should probably just be a single function rather than a struct given how compact I've refactored it to be
pub struct Decoder {
    file: QoiFile,
}

impl Decoder {
    pub fn new(input_path: &str) -> eyre::Result<Self> {
        let file = std::fs::read(input_path)?;
        let ref mut iter = file.into_iter();

        let file = QoiFile::new(iter)?;

        Ok(Self { file })
    }

    pub fn decode(&self, output_path: &str) -> eyre::Result<()> {
        let file = File::create(output_path)?;
        let writer = BufWriter::new(file);

        let mut encoder =
            png::Encoder::new(writer, self.file.header.width, self.file.header.height);

        encoder.set_color(match self.file.header.channel {
            QoiChannel::Rgb => png::ColorType::Rgb,
            QoiChannel::Rgba => png::ColorType::Rgba,
        });

        let ref mut writer = encoder.write_header()?;

        // Current pixels - used to dump into a png file later
        let mut pixels: Vec<Pixel> = vec![];
        // Pixels we've seen so far, as the spec mentions, this is meant to be 64 zero-initialized elements
        // Lookup is done using pixel.get_index_position() 
        let mut seen_pixels = [Pixel::new(); 64];
        // The previous pixel we've seen
        let mut previous_pixel: Option<Pixel> = None;

        pixels.reserve(self.file.header.get_pixel_count());

        for op in &self.file.ops {
            match op {
                // OpCode used for an incoming RGB pixel
                QoiOp::RGB(op) => {
                    let pixel = op.pixel.as_pixel();
                    pixels.push(pixel);
                    seen_pixels[pixel.get_index_position()] = pixel;
                    previous_pixel = Some(pixel);
                }

                // OpCode used for an incoming RGBA pixel
                QoiOp::RGBA(op) => {
                    let pixel = op.pixel.as_pixel();
                    pixels.push(pixel);
                    seen_pixels[pixel.get_index_position()] = pixel;
                    previous_pixel = Some(pixel);
                }

                // OpCode for looking up a pixel that has appeared previously
                // Technically not meant to see 2 consecutive ones - but that would be an encoder fault
                QoiOp::Index(op) => {
                    let pixel = seen_pixels[op.index as usize];
                    pixels.push(pixel);
                    previous_pixel = Some(pixel);
                }

                // OpCode used for computing the next pixel from the previous based off of value differences
                QoiOp::Diff(op) => {
                    let pixel = previous_pixel.expect("Expected this to not be the first pixel");
                    let pixel = pixel.from_diff(op.diff);

                    pixels.push(pixel);
                    seen_pixels[pixel.get_index_position()] = pixel;
                    previous_pixel = Some(pixel);
                }

                // Similar to Diff, but enables bigger differences
                QoiOp::Luma(op) => {
                    let pixel = previous_pixel.expect("Expected this to not be the first pixel");
                    let pixel = pixel.from_diff(op.diff);

                    pixels.push(pixel);
                    seen_pixels[pixel.get_index_position()] = pixel;
                    previous_pixel = Some(pixel);
                }

                // "The previous pixel repeats itself for run_length"
                QoiOp::Run(op) => {
                    for _ in 0..op.run_length {
                        let pixel =
                            previous_pixel.expect("Expected this to not be the first pixel");
                        pixels.push(pixel);
                    }
                }
            }
        }

        // Check if we actually filled the entire pixels vector, otherwise throw
        if pixels.len() != self.file.header.get_pixel_count() {
            return Err(eyre::eyre!(
                "actual pixel count ({}) does not match expected pixel count ({})",
                pixels.len(),
                self.file.header.get_pixel_count()
            ));
        }

        writer.write_image_data(
            &pixels
                .into_iter()
                .flat_map(|pixel| match pixel {
                    Pixel::RGB(pixel) => [pixel.r, pixel.g, pixel.b].to_vec(),
                    Pixel::RGBA(pixel) => [pixel.r, pixel.g, pixel.b, pixel.a].to_vec(),
                })
                .collect::<Vec<u8>>(),
        )?;

        Ok(())
    }
}
