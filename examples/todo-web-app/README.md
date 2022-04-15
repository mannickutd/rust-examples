# Introduction

Simple TODO web app offering CRUD endpoints connected to a postgres database.

## Getting started

Check the Cargo.toml for the required rust and dependency versions.

## Running migrations

Migrations are run using refinery_cli see refinery.toml connection details.

```
refinery migrate -c refinery.toml -p ./migrations
```
or 
```
refinery migrate -e DB_URI -p ./migrations
```

## Build

```
cargo build
```

## Run local

```
cargo run
```

## Testing

Run the unit tests

```
dropdb test-todo-web-app
createdb tests-todo-web-app
refinery migrate -c refinery_test.toml -p ./migrations
```

```
cargo test
```

## Build

```
cargo build --release
```
