use crate::common::header::QoiHeader;
use crate::common::op::{QoiOp, QoiOpDiff, QoiOpIndex, QoiOpLuma, QoiOpRGB, QoiOpRGBA, QoiOpRun};
use crate::common::pixel::{RGBAPixel, RGBPixel};
use crate::util::read_next_u8;
use std::iter::Iterator;

pub struct QoiFile {
    pub header: QoiHeader,
    pub ops: Vec<QoiOp>,
}

impl QoiFile {
    fn is_qoi_file(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<()> {
        let ref magic = [
            read_next_u8(iter)?,
            read_next_u8(iter)?,
            read_next_u8(iter)?,
            read_next_u8(iter)?,
        ];

        if magic != b"qoif" {
            return Err(eyre::eyre!("Not a valid Qoi file"));
        }

        Ok(())
    }

    // End of the data stream is marked by 7 0x00 bytes, followed by a single 0x01 byte.
    fn read_all_data(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<Vec<u8>> {
        let mut data: Vec<u8> = vec![];
        let mut chained_null_bytes = 0;

        loop {
            let byte = read_next_u8(iter)?;

            if chained_null_bytes == 7 && byte == 0x01 {
                return Ok(data);
            }

            if byte == 0x00 {
                chained_null_bytes += 1;
            } else {
                for _ in 0..chained_null_bytes {
                    data.push(0x00);
                }

                data.push(byte);
                chained_null_bytes = 0;
            }
        }
    }

    fn parse_ops(data: Vec<u8>) -> eyre::Result<Vec<QoiOp>> {
        let ref mut iter = data.into_iter();
        let mut ops: Vec<QoiOp> = vec![];

        while let Some(op) = iter.next() {
            // First check for 8-bit tags
            match op {
                // QOI_OP_RGB - next 3 bytes are R, G, and B values respectively
                0b11111110 => {
                    let pixel = RGBPixel::new(iter)?;
                    ops.push(QoiOp::RGB(QoiOpRGB { pixel }));
                }

                // QOI_OP_RGBA - next 4 bytes are R, G, B, and A values respectively
                0b11111111 => {
                    let pixel = RGBAPixel::new(iter)?;
                    ops.push(QoiOp::RGBA(QoiOpRGBA { pixel }));
                }

                // Now account for 2-bit tags
                byte => {
                    let op = (byte >> 6) & 0b11;
                    let byte = byte & 0b111111;

                    match op {
                        // QOI_OP_INDEX - remaining 6 bits of current byte are the index value
                        0b00 => {
                            ops.push(QoiOp::Index(QoiOpIndex { index: byte }));
                        }

                        // QOI_OP_DIFF - remaining 6 bits of current byte are dr, dg, and db, 2 bits each
                        0b01 => {
                            let dr = byte & 0b11;
                            let dg = (byte >> 2) & 0b11;
                            let db = (byte >> 4) & 0b11;

                            ops.push(QoiOp::Diff(QoiOpDiff {
                                // These all have a bias of 2
                                diff: RGBPixel {
                                    r: dr - 2,
                                    g: dg - 2,
                                    b: db - 2,
                                },
                            }));
                        }

                        // QOI_OP_LUMA - remaining 6 bits of current byte are dg, next byte is dr - dg and db - dg, 4 bits each
                        0b10 => {
                            // The green channel has a bias of 32, while the others have a bias of 8
                            let dg = byte - 32;
                            let next_byte = read_next_u8(iter)?;

                            let dr = next_byte & 0b1111;
                            let dr = dr - 8;
                            let dr = dr + dg;

                            let db = (next_byte >> 4) & 0b1111;
                            let db = db - 8;
                            let db = db + dg;

                            ops.push(QoiOp::Luma(QoiOpLuma {
                                diff: RGBPixel {
                                    r: dg,
                                    g: dr,
                                    b: db,
                                },
                            }));
                        }

                        // QOI_OP_RUN - remaining 6 bits of current byte are the run length
                        0b11 => {
                            ops.push(QoiOp::Run(QoiOpRun {
                                // This has a bias of -1
                                run_length: byte + 1,
                            }));
                        }

                        op => return Err(eyre::eyre!("Unknown op code: {}", op)),
                    }
                }
            }
        }

        Ok(ops)
    }

    pub fn new(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<Self> {
        QoiFile::is_qoi_file(iter)?;

        let header = QoiHeader::new(iter)?;
        let data = QoiFile::read_all_data(iter)?;
        let ops = QoiFile::parse_ops(data)?;

        Ok(Self { header, ops })
    }
}

#[cfg(test)]
mod tests {
    use super::QoiFile;

    #[test]
    fn is_qoi_file() {
        let iter: Vec<u8> = vec![0x71, 0x6F, 0x69, 0x66];
        let ref mut iter = iter.into_iter();
        QoiFile::is_qoi_file(iter).unwrap();
    }

    #[test]
    #[allow(unused_must_use)]
    fn is_not_qoi_file() {
        let iter: Vec<u8> = vec![1, 2, 3, 4];
        let ref mut iter = iter.into_iter();

        QoiFile::is_qoi_file(iter).unwrap_err();
    }

    #[test]
    fn read_all_data_valid() {
        let iter: Vec<u8> = vec![1, 2, 3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        let ref mut iter = iter.into_iter();

        let data = QoiFile::read_all_data(iter).unwrap();
        assert_eq!(vec![1, 2, 3], data);
    }

    #[test]
    #[allow(unused_must_use)]
    fn read_all_data_no_eof() {
        let iter: Vec<u8> = vec![1, 2, 3];
        let ref mut iter = iter.into_iter();

        QoiFile::read_all_data(iter).unwrap_err();
    }
}
