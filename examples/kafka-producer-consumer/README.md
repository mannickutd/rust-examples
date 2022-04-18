# Introduction

Command line kafka producer and consumer example.

## Getting started

Check the Cargo.toml for the required rust and dependency versions.

## Dependencies

Kafka needs to be running for the producer and consumer to connect to.
See [https://kafka.apache.org/quickstart](https://kafka.apache.org/quickstart)
Make sure to have created a topic which is used by the consumer and producer.

## Build

```
cargo build
``` 

## Run local

What are the command line arguments

```
./target/debug/kafka-producer-consumer -h 
```

## Run Consumer

```
RUST_LOG=debug ./target/debug/kafka-producer-consumer -c consumer -b localhost:9092 -t rust-example 
```

## Run Producer

```
RUST_LOG=debug ./target/debug/kafka-producer-consumer -c producer -b localhost:9092 -t rust-example 
```
