#[allow(dead_code)]
pub fn read_varint(buf: &Vec<u8>, offset: usize) -> u64 {
    let mut res = 0u64;

    let mut curr = buf[offset];
    let mut shift = 0u64;
    let mut read_more = true;

    while read_more {
        if buf[offset] >= 0x80u8 {
            curr ^= 0x80u8;
            read_more = true;
        } else {
            read_more = false;
        }

        res |= (curr as u64) << shift;
        shift += 7;
    }

    res
}

#[allow(dead_code)]
pub fn varint_to_vec(v: u64) -> Vec<u8> {
    let mut v = v;
    let mut res: Vec<u8> = vec![];

    loop {
        let mut curr = (v & 0xFF) as u8;
        if curr >= 0x80 {
            curr |= 0x80;
        }
        res.push(curr);
        v >>= 7;

        if v <= 0 {
            break;
        }
    }

    res
}
