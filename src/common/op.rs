use crate::common::pixel::{RGBAPixel, RGBPixel};

#[derive(Debug)]
pub struct QoiOpRGB {
    pub pixel: RGBPixel,
}

#[derive(Debug)]
pub struct QoiOpRGBA {
    pub pixel: RGBAPixel,
}

#[derive(Debug)]
pub struct QoiOpIndex {
    pub index: u8,
}

#[derive(Debug)]
pub struct QoiOpDiff {
    pub diff: RGBPixel,
}

#[derive(Debug)]
pub struct QoiOpLuma {
    pub diff: RGBPixel,
}

#[derive(Debug)]
pub struct QoiOpRun {
    pub run_length: u8,
}

#[derive(Debug)]
pub enum QoiOp {
    RGB(QoiOpRGB),
    RGBA(QoiOpRGBA),
    Index(QoiOpIndex),
    Diff(QoiOpDiff),
    Luma(QoiOpLuma),
    Run(QoiOpRun),
}
