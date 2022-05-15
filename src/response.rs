#[macro_export]
macro_rules! get_header_body_utf8 {
    ($res: expr, $socket: expr) => {
        match str::from_utf8($res.get_header_body()) {
            Ok(v) => v,
            Err(e) => return $socket.send_500(e)
        }
    }
}
#[macro_export]
macro_rules! get_body_utf8 {
    ($res: expr, $socket: expr) => {
        match str::from_utf8($res.get_body()) {
            Ok(v) => v,
            Err(e) => return $socket.send_500(e)
        }
    }
}

pub trait Response {
    fn get_body(&self) -> &[u8];
    fn get_body_formated(&self) -> Vec<&[u8]>;
    fn get_header_body(&self) -> &[u8];
    fn get_header_body_formated(&self) -> Vec<&[u8]>;
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
    //get values splitted by '|'
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
    //get body from 'Body' header
    fn get_header_body(&self) -> &[u8] {
        let l = self.len() - 1;
        let mut i = l;
        if i == 0 {return &[]}
        i -= 1;
        loop  {
            if self[i] == b'B' && self[i+1] == b'o' && self[i+2] == b'd' && self[i+3] == b'y' {
                i += 6;
                let mut j = i;
                while self[j] != b'\n' && j <= l {
                    j += 1
                }
                return &self[i..j-1]
            }else if i <= 1 {
                break
            }else {
                i -= 1;
            }
        }
        return &[]
    }
    //get body from 'Body' header splitted by '|'
    fn get_header_body_formated(&self) -> Vec<&[u8]> {
        let l = self.len() - 1;
        let mut i = l;
        if i == 0 {return vec![]}
        i -= 1;
        loop  {
            if self[i] == b'B' && self[i+1] == b'o' && self[i+2] == b'd' && self[i+3] == b'y' {
                i += 6;
                let mut res = vec![];
                let mut j = i;
                while self[j] != b'\n' && j <= l {
                    if self[j] == b'|' {
                        res.push(&self[i..j-1])
                    }
                    j += 1
                }
                return res
            }else if i <= 1 {
                break
            }else {
                i -= 1;
            }
        }
        return vec![]
    }
}
