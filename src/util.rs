pub fn read_next_u8(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<u8> {
    match iter.next() {
        Some(x) => Ok(x),
        None => Err(eyre::eyre!("Unexpected end of input")),
    }
}

pub fn read_next_u32(iter: &mut impl Iterator<Item = u8>) -> eyre::Result<u32> {
    let mut data = [0u8; 4];
    for i in 0..4 {
        data[i] = read_next_u8(iter)?;
    }

    Ok(u32::from_be_bytes(data))
}
