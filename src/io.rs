use crate::Tale;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_from_file(file_path : &str) ->  io::Result<Tale>{
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? != 0 {
        let c = get_keyword(&buffer);
    }
    Err(io::Error::new(io::ErrorKind::Other, "boop"))
}

pub fn get_keyword(line : &str) -> String {
    let mut iter = line.trim().chars();
    let r : String = iter.take_while(|x| *x != ':' && *x != '{').collect();
    r.trim().to_string()

}