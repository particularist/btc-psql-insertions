extern crate postgres;
extern crate flate2;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use flate2::read::GzDecoder;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io;
use std::process;
use std::io::prelude::*;

#[derive(Debug,Deserialize)]
struct Trade {
    id: i64, 
    price: f32,
    amount: f64
}

fn some() -> Result<(), Box<Error>> {
    let mut rdr = csv::Reader::from_path("src/resources/test.csv.gz").expect("wut?");
    for result in rdr.deserialize() {
        let trade: Trade = result?;
        println!("{:?}", trade);
    }

    Ok(())
}

fn read_string_from_gzip_file(filename: String) -> Result<String, Box<Error>> {
    let mut file = File::open(filename).expect("something happened");
    let mut buffer = Vec::new();
    // let mut buf = String::new();
    file.read_to_end(&mut buffer);
    // file.read_to_string(&mut buf).expect("huh");

    let mut gz = GzDecoder::new(&buffer[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s).expect("this aint no gzip");
    Ok(s)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn read_gz_test() {

    let decomped = read_string_from_gzip_file("./src/resources/test.csv.gz".to_owned()).unwrap();
    println!("{:?}",decomped);
    assert_eq!(decomped.len(), 430);
}
