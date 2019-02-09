# btc-psql-insertions

This project was created to read CSV data from the [BitcoinCharts.com historical trade API](http://api.bitcoincharts.com/v1/csv/).

# Building and running locally

Running this project consists of running the following command:
```
cargo run -- localhost 5433 postgres postgres --filename bitxNGN.csv.gz
```
The arguments past the `--` in the preceding command consist of `<db_host> <db_post> <db_name> <db_port> --filename`.

# Setting up the Docker postgres environment 
The `.travis.yml` file contains a `before_install` section which sets up the postgres database during the travis build. 
These commands can be used to setup a local postgres environment. This requires `Docker` and a postgres client to 
be installed locally.




| Build Status |
| ------------ |
| [![Build Status](https://travis-ci.org/particularist/btc-psql-insertions.svg?branch=master)](https://travis-ci.org/particularist/btc-psql-insertions) | 
