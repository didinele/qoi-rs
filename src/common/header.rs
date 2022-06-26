use crate::util::{read_next_u32, read_next_u8};
use std::convert::TryFrom;

#[repr(u8)]
pub enum QoiChannel {
    Rgb,
    Rgba,
}

impl TryFrom<u8> for QoiChannel {
    type Error = eyre::Error;

    fn try_from(value: u8) -> eyre::Result<Self> {
        match value {
            3 => Ok(QoiChannel::Rgb),
            4 => Ok(QoiChannel::Rgba),
            _ => Err(eyre::eyre!("Unknown channel type: {}", value)),
        }
    }
}

#[repr(u8)]
pub enum QoiColorSpace {
    Srgb,
    Linear,
}

impl TryFrom<u8> for QoiColorSpace {
    type Error = eyre::Error;

    fn try_from(value: u8) -> eyre::Result<Self> {
        match value {
            0 => Ok(QoiColorSpace::Srgb),
            1 => Ok(QoiColorSpace::Linear),
            _ => Err(eyre::eyre!("Unknown color space: {}", value)),
        }
    }
}

pub struct QoiHeader {
    pub width: u32,
    pub height: u32,
    pub channel: QoiChannel,
    pub color_space: QoiColorSpace,
}

impl QoiHeader {
    #[inline]
    pub fn get_pixel_count(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn new(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<Self> {
        let width = read_next_u32(iter)?;
        let height = read_next_u32(iter)?;
        let channel = QoiChannel::try_from(read_next_u8(iter)?)?;
        let color_space = QoiColorSpace::try_from(read_next_u8(iter)?)?;

        Ok(Self {
            width,
            height,
            channel,
            color_space,
        })
    }
}
