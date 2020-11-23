#[derive(Debug)]
pub struct Message {
    pub tag: u8,
    pub data: Vec<u8>,
}

impl Message {
    pub fn parse(buf: &Vec<u8>, offset: usize) -> Option<(usize, Message)> {
        if buf.len() < offset+3 {
            return None;
        }

        let len = ((buf[offset] as usize) << 8) | buf[offset+1] as usize;

        if len == 0 {
            return None;
        }

        let tag = buf[offset+2];
        let data = buf[offset+3..offset+3+len].to_vec();

        Some((3+len, Message{tag, data}))
    }
}