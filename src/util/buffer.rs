use std::error::Error;

pub fn check_buf_len(buf: &Vec<u8>, expected_min_len: usize) -> Result<(), Box<dyn Error>> {
    if buf.len() < expected_min_len {
        Err("invalid packet length".into())
    } else {
        Ok(())
    }
}
