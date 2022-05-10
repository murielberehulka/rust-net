use mio::{Interest, Token};
use mio::net::TcpStream;
use std::collections::HashMap;
use std::io::{Read, Write, ErrorKind::WouldBlock};
use std::net::{Shutdown};
use super::File;

const MAX_PAYLOAD_LENGTH: usize = 128;
const MAX_PAYLOADS: usize = 5;

const RES_200_0: &'static[u8] = b"HTTP/1.1 200 OK\r\nContent-Length: ";
const RES_200_1: &'static[u8] = b"\r\n\r\n";
const RES_200_LENGTH: usize = RES_200_0.len() + RES_200_1.len();

const RES_400_0: &'static[u8] = b"HTTP/1.1 400 BadRequest\r\nContent-Length: ";
const RES_400_1: &'static[u8] = b"\r\n\r\n";
const RES_400_LENGTH: usize = RES_400_0.len() + RES_400_1.len();

const RES_404: &'static[u8] = b"HTTP/1.1 404 NotFound\r\n\r\n";

const RES_500_0: &'static[u8] = b"HTTP/1.1 500 BadRequest\r\nContent-Length: ";
const RES_500_1: &'static[u8] = b"\r\n\r\n";
const RES_500_LENGTH: usize = RES_500_0.len() + RES_500_1.len();

const RES_FILE_0: &'static[u8] = b"HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: ";
const RES_FILE_1: &'static[u8] = b"\r\nContent-Length: ";
const RES_FILE_2: &'static[u8] = b"\r\n\r\n";
const RES_FILE_LENGTH: usize = RES_FILE_0.len() + RES_FILE_1.len() + RES_FILE_2.len();

pub trait Socket {
    fn read_stream(&mut self) -> Option<Vec<u8>>;
    fn send_file(&mut self, file: &File);
    fn send_200(&mut self, data: &[u8]);
    fn send_400(&mut self, data: &[u8]);
    fn send_404(&mut self);
    fn send_500(&mut self, data: &[u8]);
}

impl Socket for TcpStream {
    fn read_stream(&mut self) -> Option<Vec<u8>> {
        let mut res: Vec<u8> = vec![];
        let mut buff = [0 as u8; MAX_PAYLOAD_LENGTH];
        let mut i = 0;
        loop {
            match self.read(&mut buff) {
                Ok(len) => {
                    res.extend_from_slice(&buff[0..len]);
                    i += 1;
                    if len < MAX_PAYLOAD_LENGTH || i >= MAX_PAYLOADS {break}
                },
                Err(ref e) => if e.kind() == WouldBlock {
                    break
                } else {
                    return None
                },
            }
        }
        return Some(res)
    }
    fn send_file(&mut self, file: &File) {
        let mut send: Vec<u8> = Vec::with_capacity(RES_FILE_LENGTH + file.0.len());
        send.extend(RES_FILE_0);
        send.extend(file.0);
        send.extend(RES_FILE_1);
        send.extend(file.1.len().to_string().as_bytes());
        send.extend(RES_FILE_2);
        send.extend(&file.1);
        if let Err(_) = self.write_all(&send) {};
    }
    fn send_200(&mut self, data: &[u8]) {
        let mut send: Vec<u8> = Vec::with_capacity(RES_200_LENGTH + data.len());
        send.extend(RES_200_0);
        send.extend(data.len().to_string().as_bytes());
        send.extend(RES_200_1);
        send.extend(data);
        if let Err(_) = self.write_all(&send) {};
    }
    fn send_400(&mut self, data: &[u8]) {
        let mut send: Vec<u8> = Vec::with_capacity(RES_400_LENGTH + data.len());
        send.extend(RES_400_0);
        send.extend(data.len().to_string().as_bytes());
        send.extend(RES_400_1);
        send.extend(data);
        if let Err(_) = self.write_all(&send) {};
    }
    fn send_404(&mut self) {
        if let Err(_) = self.write_all(RES_404) {};
    }
    fn send_500(&mut self, data: &[u8]) {
        let mut send: Vec<u8> = Vec::with_capacity(RES_500_LENGTH + data.len());
        send.extend(RES_500_0);
        send.extend(data.len().to_string().as_bytes());
        send.extend(RES_500_1);
        send.extend(data);
        if let Err(_) = self.write_all(&send) {};
    }
}

pub struct Sockets (HashMap<usize, TcpStream>);

impl Sockets {
    pub fn new() -> Self {
        Self (HashMap::new())
    }
    pub fn insert(&mut self, poll: &mio::Poll, mut socket: TcpStream) {
        let token = self.0.len() + 1;
        poll.registry().register(
            &mut socket,
            Token(token),
            Interest::READABLE
        ).unwrap();
        self.0.insert(token, socket);
    }
    pub fn remove(&mut self, poll: &mio::Poll, token: usize) {
        if let Some(socket) = self.0.get_mut(&token) {
            poll.registry().deregister(socket).unwrap();
            socket.shutdown(Shutdown::Both).unwrap();
        }
        self.0.remove(&token);
    }
    pub fn get_mut(&mut self, token: usize) -> Option<&mut TcpStream> {
        self.0.get_mut(&token)
    }
}
