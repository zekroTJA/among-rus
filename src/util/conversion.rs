use std::error::Error;

pub fn vec_to_nonce_unsafe(buf: &Vec<u8>, offset: usize) -> u16 {
    ((buf[offset] as u16) << 8) | buf[offset + 1] as u16
}

pub fn nonce_to_bytes(nonce: u16) -> (u8, u8) {
    ((nonce >> 8) as u8, nonce as u8)
}

pub fn vec_to_string(buf: &Vec<u8>, offset: usize) -> Result<String, Box<dyn Error>> {
    if buf.len() < offset + 1 {
        return Err("null string len".into());
    }

    let len = buf[offset] as usize;
    if buf.len() < offset + len + 1 {
        return Err("string buffer too short".into());
    }

    let mut res = String::new();
    for i in offset + 1..=offset + len {
        res.push(buf[i] as char);
    }

    Ok(res)
}
