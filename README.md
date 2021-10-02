# A DB writer example in rust



## Setup
Ensure you have installed [Rust](https://www.rust-lang.org/tools/install) and have a working PostgreSQL
instance running on your machine. Now make sure that there is a copy of libpq (the PostgreSQL driver library)
on your machine. On Linux, install it using
```sh
sudo apt install libpq-dev
```
Now install diesel's CLI, create the database and run the migration:
```sh
cargo install diesel_cli --no-default-features --features postgres   # Only install PostgreSQL components
createdb dbwriter_rust   # Create the required database in prostgres
echo DATABASE_URL=postgres://username:password@localhost/dbwriter_rust > .env  # Diesel gets the connection string from here
diesel migration run  # Roll the actual migration and generate a Rust schena that matches the current database schema
```

To start the server, just type `cargo run`