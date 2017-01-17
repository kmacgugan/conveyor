extern crate amqp;
extern crate env_logger;
extern crate serde;
extern crate serde_json;

use self::amqp::{Session, Options, Table, Basic, protocol, Channel, ConsumerCallBackFn};
use std::default::Default;
use elastic_output;


fn consumer_function(channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
    println!("[function] Got a delivery:");
    println!("[function] Deliver info: {:?}", deliver);
    println!("[function] Content headers: {:?}", headers);
    let body_as_string = String::from_utf8(body).unwrap();
    println!("[function] Content body(as string): {:?}", body_as_string);
    let habitat_data : elastic_output::Census = parse_census(&body_as_string);
    elastic_output::publish_habitat(habitat_data.ring_id, body_as_string);
    channel.basic_ack(deliver.delivery_tag, false).unwrap();
}

pub fn parse_census(contents: &String) -> elastic_output::Census {
    let census : elastic_output::Census = match serde_json::from_str(&contents) {
        Err(why) => panic!("Couldn't serialize json: {}", why),
        Ok(census) => census,
    };
    println!("Ring id is {}", census.ring_id);
    census
}

pub fn consume() {
    env_logger::init().unwrap();
    let mut session = Session::new(Options{ host: "rabbitmq".to_string(),
         login: "insights".to_string(), password: "chefrocks".to_string(), vhost: "/insights".to_string(), .. Default::default()}).ok().expect("Can't create session");
    let mut channel = session.open_channel(1).ok().expect("Error openning channel 1");
    println!("Openned channel: {:?}", channel.id);

    let queue_name = "habitat-collector";
    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());

    println!("Queue declare: {:?}", queue_declare);
    //
    //let exchange_declare1 = channel.exchange_declare("habitat", "topic",
    //                                                 false, true, false, false, false, Table::new());
    //println!("Exchange declare: {:?}", exchange_declare1);
    //
    //let bind_reply = channel.queue_bind("habitat-collector", "habitat", "#", true, Table::new());
    //println!("Exchange bind: {:?}", bind_reply);

    channel.basic_prefetch(10).ok().expect("Failed to prefetch");

    //consumer, queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    println!("Declaring consumer...");
    let consumer_name = channel.basic_consume(consumer_function as ConsumerCallBackFn, queue_name, "", false, false, false, false, Table::new());
    println!("Starting consumer {:?}", consumer_name);

    //let consumers_thread = thread::spawn(move || {
        channel.start_consuming();
    //    channel
//    });

    // There is currently no way to stop the consumers, so we infinitely join thread.
//    let mut channel = consumers_thread.join().ok().expect("Can't get channel from consumer thread");

    channel.close(200, "Bye").unwrap();
    session.close(200, "Good Bye");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io;
    #[test]
    fn parses_json() {
        let file_result = read_file("/Users/kmacgugan/chef/conveyor/app/data/census.json");
        let contents = match file_result {
            Err(why) => panic!("couldn't read file: {}", why),
            Ok(contents) => contents,
        };
        parse_census(contents);
    }

    fn read_file(filename: &str) -> io::Result<String>  {
        let mut f = try!(File::open(filename));
        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));
        Ok(contents)
    }
}
