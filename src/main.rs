#![feature(proc_macro)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::io::prelude::*;
use std::fs::File;
use std::io;

//extern crate serde_derive;
//extern crate serde_json;
mod census;
use census::Census;

fn main() {
    let file_result = read_file("/Users/kmacgugan/chef/conveyor/data/census.json");
    let contents = match file_result {
        Err(why) => panic!("couldn't read file: {}", why),
        Ok(contents) => contents,
    };
    parse_census(&contents);
}

fn read_file(filename: &str) -> io::Result<String>  {
    let mut f = try!(File::open(filename));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(contents)
}

fn parse_census(contents: &String) -> io::Result<()>  {
    let census : Census = match serde_json::from_str(&contents) {
        Err(why) => panic!("Couldn't serialize json: {}", why),
        Ok(census) => census,
    };
    println!("Ring id is {}", census.ring_id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json() {

    }
}
