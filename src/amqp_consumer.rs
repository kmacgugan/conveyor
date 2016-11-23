extern crate amqp;
extern crate env_logger;

use amqp::{Session, Options, Table, Basic, protocol, Channel, ConsumerCallBackFn};
use amqp::protocol::basic;
use std::default::Default;

fn consumer_function(channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
    println!("[function] Got a delivery:");
    println!("[function] Deliver info: {:?}", deliver);
    println!("[function] Content headers: {:?}", headers);
    println!("[function] Content body: {:?}", body);
    println!("[function] Content body(as string): {:?}", String::from_utf8(body));
    channel.basic_ack(deliver.delivery_tag, false).unwrap();
}

struct MyConsumer {
    deliveries_number: u64
}

impl amqp::Consumer for MyConsumer {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, headers: protocol::basic::BasicProperties, body: Vec<u8>){
        println!("[struct] Got a delivery # {}", self.deliveries_number);
        println!("[struct] Deliver info: {:?}", deliver);
        println!("[struct] Content headers: {:?}", headers);
        println!("[struct] Content body: {:?}", body);
        println!("[struct] Content body(as string): {:?}", String::from_utf8(body));
        // DO SOME JOB:
        self.deliveries_number += 1;
        channel.basic_ack(deliver.delivery_tag, false).unwrap();
    }
}

fn consume() {
    env_logger::init().unwrap();
    let mut session = Session::new(Options{ vhost: "/".to_string(), .. Default::default()}).ok().expect("Can't create session");
    let mut channel = session.open_channel(1).ok().expect("Error openning channel 1");
    println!("Openned channel: {:?}", channel.id);

    let queue_name = "habitat";
    //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
    let queue_declare = channel.queue_declare(queue_name, false, true, false, false, false, Table::new());

    println!("Queue declare: {:?}", queue_declare);
    channel.basic_prefetch(10).ok().expect("Failed to prefetch");
    //consumer, queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
    println!("Declaring consumers...");

    let consumer_name = channel.basic_consume(consumer_function  as ConsumerCallBackFn, queue_name, "", false, false, false, false, Table::new());
    println!("Starting consumer {:?}", consumer_name);

    let my_consumer = MyConsumer { deliveries_number: 0 };
    let consumer_name = channel.basic_consume(my_consumer, queue_name, "", false, false, false, false, Table::new());
    println!("Starting consumer {:?}", consumer_name);

    let mut delivery_log = vec![];
    let closure_consumer = Box::new(move |_chan: &mut Channel, deliver: basic::Deliver, headers: basic::BasicProperties, data: Vec<u8>|
        {
            println!("[closure] Deliver info: {:?}", deliver);
            println!("[closure] Content headers: {:?}", headers);
            println!("[closure] Content body: {:?}", data);
            delivery_log.push(deliver);
        }
    );
    let consumer_name = channel.basic_consume(closure_consumer, queue_name, "", false, false, false, false, Table::new());
    println!("Starting consumer {:?}", consumer_name);

    channel.start_consuming();

    channel.close(200, "Bye").unwrap();
    session.close(200, "Good Bye");
}
