use crate::*;
use std::io;
use std::fs::{File};
use std::io::{BufReader, BufRead};
use log;
use std::collections::HashMap;

struct TaleBuilder {
    attributes : HashMap<String, String>
}

impl TaleBuilder {
    fn new() -> TaleBuilder {
        TaleBuilder {
            attributes : HashMap::new()
        }
    }

    fn build(&mut self, reader : &mut BufReader<File>, first_line : &str) {
        self.attributes = get_attributes(reader, &mut String::new(), '}');
    }

    fn finalize(self,
        config_builder : ConfigBuilder,
        chapter_collection_builder : ChapterCollectionBuilder) -> io::Result<Tale> {
        log::info!("Finalizing Tale ...");
        Ok(Tale {
            attributes : self.attributes,
            start: Chapter::new(),
            config: Config::new()
        })
    }
}

struct ConfigBuilder {}

impl ConfigBuilder {
    fn new() -> ConfigBuilder {
        ConfigBuilder {}
    }

    fn build(&mut self, reader : &mut BufReader<File>, first_line : &str) {}
}

struct ChapterCollectionBuilder {}

impl ChapterCollectionBuilder {
    fn new() -> ChapterCollectionBuilder {
        ChapterCollectionBuilder {}
    }

    fn build_chapter(&mut self, reader : &mut BufReader<File>, first_line : &str) {}
}

pub fn read_from_file(file_path: &str) -> io::Result<Tale> {
    read_ff(file_path, false)
}

pub fn read_from_file_full(file_path: &str) -> io::Result<Tale> {
    read_ff(file_path, true)
}


pub fn read_ff(file_path: &str, _ : bool) -> io::Result<Tale> {
    let f = File::open(file_path).expect(format!("File not found: {}", file_path).as_ref());
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut tale_builder = TaleBuilder::new();
    let mut config_builder = ConfigBuilder::new();
    let mut chapter_collection_builder = ChapterCollectionBuilder::new();

    log::info!("Starting build ...");

    while reader.read_line(&mut buffer)? != 0 {
        match get_key_value(&buffer, vec!['{']) {
            None => {continue}
            Some((key, value)) => {
                match key.as_ref() {
                    "tale" => {
                        tale_builder.build(&mut reader, &value);
                    },
                    "config" => {
                        config_builder.build(&mut reader, &value);
                    },
                    "chapter" => {
                        chapter_collection_builder.build_chapter(&mut reader, &value);
                    },
                    _ => {}
                }
            }
        }
    }
    tale_builder.finalize(config_builder, chapter_collection_builder)
}

pub fn get_key_value(line: &str, mut separators : Vec<char>) -> Option<(String, String)> {
    let line = line.trim();
    match find_first_separator(line, separators) {
        None => None,
        Some(i) => Some((line[..i].trim().to_string(), line[i+1..].trim().to_string()))
    }
}

/// Returns position of the first separator or `None` if no separator is in the string.
fn find_first_separator(s: &str, mut separators : Vec<char>) -> Option<usize> {
    let mut iter = s.char_indices();
    let mut first = None;
    while !separators.is_empty() {
        let sep = match separators.pop() {
            Some(c) => c,
            _ => { continue; }
        };
        match iter.clone().find(|x| { x.1 == sep }) {
            Some(f) => {
                if first.is_none() { first = Some(f.0) }
                if first? > f.0 { first = Some(f.0) }
            }
            None => {}
        }
    }
    return first;
}

fn get_attributes(reader : &mut BufReader<File>, first_line : &mut String, end_char : char) -> HashMap<String, String>{
    let mut map : HashMap<String, String> = HashMap::new();
    let mut buffer = first_line.clone();
    while buffer.char_indices().find(|x| x.1 == end_char).is_none() {
        match get_key_value(&buffer, vec![':']) {
            None => {
                buffer = String::new();
                reader.read_line(&mut buffer);
                continue;
            }
            Some((k, v)) => {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
        buffer = String::new();
        reader.read_line(&mut buffer);
    }
    map
}

