use storyteller::io::{get_key_value, read_from_file};
use simplelog::{SimpleLogger, Config};
use log::LevelFilter;

fn main() {
    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

    let t = read_from_file("res-examples/test");
    println!{"{}", t.unwrap()}
}