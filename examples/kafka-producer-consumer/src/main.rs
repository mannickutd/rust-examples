use std::io::Write;
use std::thread;
use clap::Parser;
use env_logger::fmt::Formatter;
use env_logger::Builder;
use log::{LevelFilter, Record};
use kafka_producer_consumer::consumer::consumer::consume_and_print;
use kafka_producer_consumer::producer::producer::produce;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help="Whether to run the kafka consumer or producer.")]
    consumer_producer: String,

    #[clap(short='b', long="brokers", help="Broker list in kafka format")]
    brokers: String,

    #[clap(short='t', long="topic", help="Destination topic")]
    topic: String
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::init();
    match args.consumer_producer.as_str() {
        "consumer" => {
            consume_and_print(&args.brokers, &"simple_consumer", &[&args.topic]).await;
        }
        "producer" => {
            produce(&args.brokers, &args.topic).await;
        }
        _ => {
            panic!("AAAAHHHhhhhh");
        }
    }
}
