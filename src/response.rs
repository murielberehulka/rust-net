#[macro_export]
macro_rules! get_body_utf8 {
    ($res: expr, $socket: expr) => {
        match str::from_utf8(res.get_body()) {
            Ok(v) => v,
            Err(e) => return on_error_500(socket, e)
        }
    };
}
pub(crate) use get_body_utf8;

pub trait Response {
    fn get_body(&self) -> &[u8];
    fn get_body_formated(&self) -> Vec<&[u8]>;
}

impl Response for Vec<u8>{
    fn get_body(&self) -> &[u8] {
        let mut i = self.len();
        if i == 0 {return &[]}
        i -= 1;
        while self[i] != b'\n' {
            i -= 1;
        }
        i += 1;
        return &self[i..self.len()]
    }
    //get values splited by '|'
    fn get_body_formated(&self) -> Vec<&[u8]> {
        let mut res = vec![];
        let mut i = self.len();
        if i == 0 {return vec![]}
        i -= 1;
        let mut j = i + 1;
        while self[i] != b'\n' {
            if self[i] == b'|' {
                res.push(&self[(i+1)..j]);
                j = i;
            }
            i -= 1;
        }
        res.push(&self[(i+1)..j]);
        res
    }
}
