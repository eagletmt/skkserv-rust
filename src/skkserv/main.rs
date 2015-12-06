extern crate encoding;

use std::io::{Read, Write};
use std::net::TcpListener;
use std::os::unix::io::AsRawFd;
use std::thread;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::{ASCII, EUC_JP};

fn main() {
    let listener = TcpListener::bind(("127.0.0.1", 11178)).unwrap();
    let encoding = EUC_JP;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    println!("Connected {}", stream.as_raw_fd());
                    let _ = handle_stream(&mut stream, encoding);
                    println!("Disconnected {}", stream.as_raw_fd());
                });
            }
            Err(e) => {
                let _ = writeln!(&mut std::io::stderr(), "{}", e);
            }
        }
    }
}

fn handle_stream<T: Read + Write>(stream: &mut T, encoding: &Encoding) -> std::io::Result<()> {
    loop {
        let mut cmd = [0; 1];
        let r = try!(stream.read(&mut cmd));
        if r == 0 {
            break;
        }
        match cmd[0] {
            0x30 => {
                break;
            }
            0x31 => {
                let _ = respond_candidate(stream, encoding);
            }
            0x32 => {
                let _ = respond_version(stream);
            }
            _ => { println!("Unknown message {}", cmd[0]); }
        }
    }
    return Ok(());
}

fn respond_version(stream: &mut Write) -> std::io::Result<()> {
    let _ = try!(write_with_encoding(stream, "skkserv-rust:0.1.0", ASCII));
    try!(stream.flush());
    return Ok(());
}

fn respond_candidate<T: Read + Write>(stream: &mut T, encoding: &Encoding) -> std::io::Result<()> {
    let input = try!(read_candidate(stream, encoding));
    println!("Asked {}", input);
    let _ = try!(write_with_encoding(stream, "1/aiueo/\n", encoding));
    return Ok(());
}

fn read_candidate(stream: &mut Read, encoding: &Encoding) -> std::io::Result<String> {
    let mut buf = Vec::new();

    loop {
        let mut t = [0; 1024];
        let r = try!(stream.read(&mut t));
        if r == 0 {
            panic!("Unterminated input");
        }
        for i in 0..r {
            if t[i] == 0x20 {
                return Ok(encoding.decode(&buf, DecoderTrap::Strict).unwrap());
            }
            buf.push(t[i]);
            // TODO: unpush rest
        }
    }
}

fn write_with_encoding(stream: &mut Write, input: &str, encoding: &Encoding) -> std::io::Result<usize> {
    return stream.write(&encoding.encode(input, EncoderTrap::Strict).unwrap());
}
