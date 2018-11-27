extern crate postgres;
extern crate flate2;

use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Trade {
    id: i64, 
    price: f32,
    amount: f64
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn a_test() {

    let mut file = File::open("./src/resources/test.csv.gz").expect("something happened");
    let mut buffer = Vec::new();
    // let mut buf = String::new();
    file.read_to_end(&mut buffer);
    // file.read_to_string(&mut buf).expect("huh");

    let mut gz = GzDecoder::new(&buffer[..]);
    let mut s = String::new();
   gz.read_to_string(&mut s).expect("this aint no gzip");
    println!("{}", s);
}
