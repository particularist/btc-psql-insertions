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
    time: i64, 
    price: f32,
    amount: f64
}

fn prepend_header(contents: String) -> Result<String, Box<Error>> {
    Ok(format!("{}\n{}", "time,price,amount", contents))
}


fn deserialize_trades_from_file_contents(contents: String) -> Result<Vec<Trade>, Box<Error>> {
    let prepended_contents = prepend_header(contents).unwrap();
    let mut vec: Vec<Trade> = Vec::new();

    let mut rdr = csv::Reader::from_reader(prepended_contents.as_bytes());
    for result in rdr.deserialize() {
        let trade: Trade = result?;
        println!("{:?}", trade);
        vec.push(trade);
    }

    Ok(vec)
}

fn read_string_from_gzip_file(filename: String) -> Result<String, Box<Error>> {
    let mut file = File::open(filename).expect("something happened");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let mut gz = GzDecoder::new(&buffer[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s).expect("this aint no gzip");
    Ok(s)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn deserialize_trades_test() {
    let contents = read_string_from_gzip_file("./src/resources/test.csv.gz".to_string()).unwrap();
    assert_eq!(contents.len(), 430);

    let trades = deserialize_trades_from_file_contents(contents).unwrap();
    assert_ne!(trades.len(), 0);
}

#[test]
fn read_gz_test() {

    let decomped = read_string_from_gzip_file("./src/resources/test.csv.gz".to_owned()).unwrap();
    println!("{:?}",decomped);
    assert_eq!(decomped.len(), 430);
}

#[test]
fn prepend_header_test() {

    let prepended = prepend_header("These, are, test\ncontents, which, look\nquite, real, actually\n".to_string()).unwrap();
    println!("{:?}", prepended);
}
