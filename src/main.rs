extern crate postgres;
extern crate flate2;
extern crate csv;
#[macro_use] 
extern crate quicli;
#[macro_use]
extern crate serde_derive;

use flate2::read::GzDecoder;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io;
use std::process;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use postgres::{Connection, TlsMode};
use quicli::prelude::*;


#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "filename", short="f", default_value = "./src/resources/test.csv.gz")]
    filename: String,
    db_hostname: String,
    db_port: i32,
    db_user: String,
    db_name: String
}

#[derive(Clone,Debug,Deserialize)]
struct Trade {
    time: i64,
    price: f32,
    amount: f32
}

fn prepend_header(contents: String) -> Result<String> {
    Ok(format!("{}\n{}", "time,price,amount", contents))
}

fn deserialize_trades_from_file_contents(contents: String) -> Result<Vec<Trade>> {
    let prepended_contents = prepend_header(contents).unwrap();
    let mut vec: Vec<Trade> = Vec::new();

    let mut rdr = csv::Reader::from_reader(prepended_contents.as_bytes());
    for result in rdr.deserialize() {
        let trade: Trade = result?;
        vec.push(trade);
    }

    Ok(vec)
}

fn read_string_from_gzip_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename).expect("something happened");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);

    let mut gz = GzDecoder::new(&buffer[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s).expect("this aint no gzip");
    Ok(s)
}

fn ins_trades(conn: &Connection, trades: Vec<Trade>, exchange: &str, currency: &str){
    for t in trades.iter() {
        conn.execute("INSERT INTO trades (exchange, currency, time, price, amount) VALUES ($1, $2, $3, $4, $5)", &[&exchange, &currency, &t.time, &t.price, &t.amount]).unwrap();
    }
}

fn extract_exchange_and_currency(filename: &str) -> (String, String) {
    let currency: Vec<&str> = filename.matches(char::is_uppercase).collect();
    let exchange: Vec<&str> = filename.matches(char::is_lowercase).collect();
    (exchange.join(""),currency.join(""))
}

main!(|args: Cli| {
    let user =  args.db_user.to_owned();
    let hostname =  args.db_hostname.to_owned();
    let port =  args.db_port.to_owned();
    {
        let num_of_threads = 4;
        let the_file = args.filename.to_owned();
        let contents = read_string_from_gzip_file(&the_file).unwrap();
        let ts = deserialize_trades_from_file_contents(contents);
        let act_trade_vec:Vec<Trade> = ts.unwrap().clone();
        {
            let handle = thread::spawn(move || {
                let mut trade_it = act_trade_vec.chunks(num_of_threads);
                let exchange_currency_tuple = extract_exchange_and_currency(&the_file);
                println!("inserting");
                trade_it.map(|trade_slices|
                    ins_trades(&Connection::connect(format!("postgres://{}@{}:{}", user, hostname, port), TlsMode::None).unwrap(), trade_slices.to_vec(), &exchange_currency_tuple.0, &exchange_currency_tuple.1)
            ).collect::<Vec<_>>();
            });
            handle.join().unwrap();
        }
    }
});

#[test]
fn closure_test() {

    let dis_x = |x| format!("{}!", x);
    let res = dis_x("sup");
    let do_the_thing = || println!("{}", res);
    do_the_thing();
    do_the_thing();
    do_the_thing();
}

#[test]
fn extract_exchange_test() {
    let tups =extract_exchange_and_currency(&"thisistheexchangeUSD");
    assert_eq!(tups.0, "thisistheexchange");
    assert_eq!(tups.1, "USD");
}

#[test]
fn insert_trades_int_test() {
    let conn = Connection::connect(format!("postgres://{}@{}:{}", "postgres", "localhost", "5433"), TlsMode::None).unwrap();
    let contents = read_string_from_gzip_file(&"./src/resources/test.csv.gz").unwrap();
    assert_eq!(contents.len(), 430);

    let trades = deserialize_trades_from_file_contents(contents).unwrap();
    assert_ne!(trades.len(), 0);
    ins_trades(&conn, trades, "an_exchange", "EUR");
    for c in &conn.query("SELECT count(1) FROM trades", &[]).unwrap() {
        let cnt: i64 = c.get(0);
        assert_eq!(cnt, 10);
    }
    conn.execute("delete from trades", &[]).unwrap();
}

#[test]
fn deserialize_trades_test() {
    let contents = read_string_from_gzip_file("./src/resources/test.csv.gz").unwrap();
    assert_eq!(contents.len(), 430);

    let trades = deserialize_trades_from_file_contents(contents).unwrap();
    assert_ne!(trades.len(), 0);
}

#[test]
fn read_gz_test() {
    let decomped = read_string_from_gzip_file(&"./src/resources/test.csv.gz").unwrap();
    println!("{:?}",decomped);
    assert_eq!(decomped.len(), 430);
}

#[test]
fn prepend_header_test() {

    let prepended = prepend_header("These, are, test\ncontents, which, look\nquite, real, actually\n".to_string()).unwrap();
    println!("{:?}", prepended);
}
