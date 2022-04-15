# Introduction

Simple TODO web app offering CRUD endpoints connected to a postgres database.

## Getting started


## Running migrations

Migrations are run using refinery_cli see refinery.toml connection details.

```
refinery migrate -c refinery.toml -p ./migrations
```
or 
```
refinery migrate -e DB_URI -p ./migrations
```


## Testing

```
cargo test
```

## Build

```
cargo build --release
```
