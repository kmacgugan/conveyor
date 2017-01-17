#[macro_use] extern crate log;
#[macro_use] extern crate hyper;
extern crate serde;
extern crate serde_json;

// This is to support stable rust 1.13. https://serde.rs/codegen-stable.html

mod elastic_output;
mod amqp_consumer;
mod elastic_initializer;

use std::env;
use std::fs;
fn main() {
    let es_url = "http://elasticsearch:9200/habitat-state";
    let curpath = env::current_dir().unwrap();
    println!("{:?}", fs::canonicalize(&curpath));
    let hab_path = "./hab/pkgs/kmacgugan/conveyor/0.0.1/";
    let mut paths = fs::read_dir(hab_path).unwrap();
    let mut hab_data_path = paths.next().unwrap().unwrap().path();
    hab_data_path.push("data/habitat-template.json");

    //let pathStr = path.to_str().unwrap();
    elastic_initializer::initialize_habitat_index(es_url, hab_data_path.to_str().unwrap());
    amqp_consumer::consume();
}
