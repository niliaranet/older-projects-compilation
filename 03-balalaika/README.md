# Balalaika
Lyrics api written in rust (school project).

## Setup
### Dependencies
This project requires mariadb and cargo.

### Set up env
Create a .env file and fill it using the env_example as a guide.

### Set up database
To manage the database through the program's scripts use
`cargo run-script`
(installable through cargo with `cargo install cargo-run-script`).

Available scripts:
```
db_create
db_fast_populate
db_start
db_stop
```

The start/stop scripts assume that you're using systemd.
If not, just start/stop the mariadb service manually.

db_fast_populate is an alternative to populate the database
without having to interact with Genius' api.
You can still use the original method by executing
./scripts/populate.main.py with a python interpreter.
