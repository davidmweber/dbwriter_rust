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
# This is only needed if the database user specified in the env variable cannot create a database
createdb dbwriter_rust   # Create the required database in prostgres
echo DATABASE_URL=postgres://username:password@localhost/dbwriter_rust > .env  # Diesel gets the connection string from here
diesel setup
diesel migration run  # Roll the actual migration and generate a Rust schena that matches the current database schema
```
To start the server, just type `cargo run`

# Benchmarking
This was tested with [wrk2](git@github.com:giltene/wrk2.git). Here is an example benchmark:
```
wrk -v -t4 -c200 -R600000 http://localhost:8080/samples/3
```
This will attempt 6000000 requests per second to the specified URL.

Some hints for Rust: 
- Install flamegraph ('cargo install flamegraph') for some top-notch profiling. Execute this using `cargo flamegraph`
- You can visualise the reslting SVG flamegraph by opening the flamegraph.svb file with Chrome or Firefox.

# The good 
The database first migration strategy works very well

# The not so good
- Diesel is not fundamentally synchronous because it relies on a thread pool for database access. I feel this is a manageable
  oversight but given Rust's excellent async options, I think they could do better. The so called async wrappers
  are just thunking down to the thread pool, so it is really just sugar.

# The painful parts
- Rust is mighty picky about references and types. It is actually hard to figure out what to do in various places
- The IDE (Jetbrains and VSCode) have mediocre support for Rust. They appear to be blind to macros and offer little 
  help with imports. The compiler will vomit up many errors that the IDE just misses.
- Figuring out what type to use when mapping structs to a database schema can be tricky. The docs are not great in this
  regard.

Alternative ReST implementations: [Rocket](https://rocket.rs/) looks decent