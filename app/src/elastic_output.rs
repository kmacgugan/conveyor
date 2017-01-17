use hyper::client::Client;

extern crate serde;
extern crate serde_json;
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));
use std::io::Read;


pub fn publish_habitat(ring_id: uuid::Uuid, census: String){
   let client = Client::new();
   let es_url = format!("http://elasticsearch:9200/habitat-state/habitat/{}", ring_id);
   println!("es_url: {}", es_url);
       let mut s = String::new();
   let res = client.put(es_url.as_str())
            .body(census.as_str())
            .send();
        res.unwrap().read_to_string(&mut s).unwrap();
        println!("Result: {}", s );
    // match res {
    //     Ok(res) => println!("Response: {}", res.status,
    //     Err(e) => println!("Err: {:?}", e)
    // }
}