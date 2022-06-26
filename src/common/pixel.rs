use crate::util::read_next_u8;

#[derive(Debug, Clone, Copy)]
pub struct RGBPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBPixel {
    pub fn get_index_position(&self) -> usize {
        let r = self.r as usize;
        let g = self.g as usize;
        let b = self.b as usize;

        ((r * 3 + g * 5 + b * 7) % 64) as usize
    }

    pub fn new(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<Self> {
        let r = read_next_u8(iter)?;
        let g = read_next_u8(iter)?;
        let b = read_next_u8(iter)?;

        Ok(Self { r, g, b })
    }

    pub fn as_pixel(self) -> Pixel {
        Pixel::RGB(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RGBAPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBAPixel {
    pub fn get_index_position(&self) -> usize {
        let r = self.r as usize;
        let g = self.g as usize;
        let b = self.b as usize;
        let a = self.a as usize;

        ((r * 3 + g * 5 + b * 7 + a * 11) % 64) as usize
    }

    pub fn new(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<Self> {
        let r = read_next_u8(iter)?;
        let g = read_next_u8(iter)?;
        let b = read_next_u8(iter)?;
        let a = read_next_u8(iter)?;

        Ok(Self { r, g, b, a })
    }

    pub fn as_pixel(self) -> Pixel {
        Pixel::RGBA(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Pixel {
    RGB(RGBPixel),
    RGBA(RGBAPixel),
}

impl Pixel {
    pub fn get_index_position(&self) -> usize {
        match self {
            Pixel::RGB(pixel) => pixel.get_index_position(),
            Pixel::RGBA(pixel) => pixel.get_index_position(),
        }
    }

    pub fn new() -> Self {
        Pixel::RGBA(RGBAPixel {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        })
    }

    pub fn from_diff(&self, diff: RGBPixel) -> Self {
        match self {
            Pixel::RGB(pixel) => {
                let r = pixel.r.wrapping_add(diff.r);
                let g = pixel.g.wrapping_add(diff.g);
                let b = pixel.b.wrapping_add(diff.b);
                Pixel::RGB(RGBPixel { r, g, b })
            }

            Pixel::RGBA(pixel) => {
                let r = pixel.r.wrapping_add(diff.r);
                let g = pixel.g.wrapping_add(diff.g);
                let b = pixel.b.wrapping_add(diff.b);
                Pixel::RGBA(RGBAPixel {
                    r,
                    g,
                    b,
                    a: pixel.a,
                })
            }
        }
    }
}
