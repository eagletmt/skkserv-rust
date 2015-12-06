extern crate encoding;
extern crate getopts;

use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use getopts::Options;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::{ASCII, EUC_JP};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("h", "host", "Host to connect", "ADDR");
    opts.optopt("p", "port", "Port to connect", "PORT");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()); }
    };
    let host = matches.opt_str("h").unwrap_or("127.0.0.1".to_string());
    let port = matches.opt_str("p").unwrap_or("1178".to_string()).parse::<u16>().unwrap();
    let default_kana = "a".to_string();
    let kana = matches.free.first().unwrap_or(&default_kana);

    println!("Connecting {}:{}", host, port);
    let mut stream = TcpStream::connect((&*host, port)).unwrap();
    let encoding = EUC_JP;

    let version = get_version(&mut stream, encoding).unwrap();
    println!("Version: {}", version);

    let candidate = get_candidate(&mut stream, &kana, encoding).unwrap();
    println!("{}", kana);
    println!("{}", candidate);

    let _ = disconnect(&mut stream).unwrap();
}

fn get_version<T: Read + Write>(stream: &mut T, encoding: &Encoding) -> std::io::Result<String> {
    try!(write_with_encoding(stream, "2", ASCII));
    try!(stream.flush());
    return read_with_encoding(stream, encoding);
}

fn disconnect<T: Write>(stream: &mut T) -> std::io::Result<()> {
    try!(write_with_encoding(stream, "0", ASCII));
    try!(stream.flush());
    return Ok(());
}

fn get_candidate<T: Read + Write>(stream: &mut T, kana: &str, encoding: &Encoding) -> std::io::Result<String> {
    try!(write_with_encoding(stream, "1", ASCII));
    try!(write_with_encoding(stream, kana, encoding));
    try!(write_with_encoding(stream, " ", ASCII));
    try!(stream.flush());
    return read_with_encoding(stream, encoding);
}

fn write_with_encoding(stream: &mut Write, input: &str, encoding: &Encoding) -> std::io::Result<usize> {
    return stream.write(&encoding.encode(input, EncoderTrap::Strict).unwrap());
}

fn read_with_encoding(stream: &mut Read, encoding: &Encoding) -> std::io::Result<String> {
    let mut buf = [0; 1024];
    let r = try!(stream.read(&mut buf));
    return Ok(encoding.decode(&buf[0 .. r-1], DecoderTrap::Strict).unwrap());
}
