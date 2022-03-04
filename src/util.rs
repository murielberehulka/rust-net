pub const U8_G: u8 = 'G' as u8;
pub const U8_P: u8 = 'P' as u8;
pub const U8_O: u8 = 'O' as u8;
pub const U8_SPACE: u8 = ' ' as u8;
pub const INDEX: &'static[u8] = &['i' as u8,'n' as u8,'d' as u8,'e' as u8,'x' as u8,'.' as u8,'h' as u8,'t' as u8,'m' as u8,'l' as u8];

pub fn read_data_until_space(data: &Vec<u8>, start: usize) -> &[u8] {
    let mut end = start;
    let max = data.len();
    while data[end] != U8_SPACE && end < max  {
        end += 1;
    }
    &data[start..end]
}

pub fn get_extension(v: &[u8]) -> &[u8] {
    let mut i = v.len() - 1;
    if i < 1 || v[i] == b'.' {return &[]}
    while i > 0 && v[i] != b'.' {
        i -= 1;
    }
    i += 1;
    &v[i..]
}