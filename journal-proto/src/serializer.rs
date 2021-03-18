use super::*;

pub fn frame_bytes(value: &[u8]) -> Result<Vec<u8>> {
    let mut res_bytes = Vec::new();

    let size_varint_bytes = vec![0u8; 0];
    // Cursor is needed, because varint does not implement VarWrite on all Write impls.
    // See https://github.com/nham/Varint-rs/tree/add_impl
    let mut cur = Cursor::new(size_varint_bytes);
    cur.write_unsigned_varint_32(value.len() as u32)?;
    let size_varint_bytes = cur.into_inner();

    res_bytes.write_all(&size_varint_bytes)?;
    res_bytes.write_all(value)?;
    Ok(res_bytes)
}

// fn unframe_bytes(bytes: Vec<u8>) -> Result<Vec<u8>> {
//     let mut cur = Cursor::new(bytes);
//     let str_length = cur.read_unsigned_varint_32()?;
//
//     let mut str_bytes = Vec::new();
//     str_bytes.resize(str_length as usize, 0u8);
//     cur.read_exact(str_bytes.as_mut_slice())?;
//
//     Ok(str_bytes)
// }
