# A DB writer example in rust
A basic modularised demo of a Rust language based API using [actix-web](https://actix.rs/) and
[Diesel](https://diesel.rs/). 

## Setup
Ensure you have installed [Rust](https://www.rust-lang.org/tools/install) and have a working PostgreSQL instance with a copy of libpq 
(the PostgreSQL driver library) on your machine. On Linux, install it using
```sh
sudo apt install libpq-dev
```
Now install diesel's CLI, create the database and run the migration noting that you may have to grant
your user access to the `dbwriter_rust` database:
```sh
cargo install diesel_cli --no-default-features --features postgres   # Only install PostgreSQL components
# This is only needed if the database user specified in the env variable cannot create a database
createdb dbwriter_rust   # Create the required database in postgres
echo DATABASE_URL=postgres://username:password@localhost/dbwriter_rust > .env  # Diesel gets the connection string from here
diesel setup
diesel migration run  # Roll the actual migration and generate a Rust schema that matches the current database schema
```

You may have to grant the user access to the database. This can be done from the psql command line. Start `psql`
```shell
psql dbwriter_rust
```
Then execute the following SQL:
```sql
GRANT ALL PRIVILEGES ON DATABASE dbwriter_rust TO test;
GRANT ALL ON SCHEMA public TO test;
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
- You can visualise the resulting SVG flamegraph by opening the flamegraph.svb file with Chrome or Firefox.

# The good 
- The database first migration strategy works very well.
- Diesel is a fully fledged ORM that is good to work with.
- It is insanely fast at 260k requests per second on my 6/12 core machine using the above wrk command.

# The not so good
- Diesel is not (yet) fundamentally synchronous because it relies on the standard blocking PostgreSQL driver.
  You will need to think carefully about how you handle database requests in an async environment.
 
- Actix works fine, but it is hard to gauge what types are needed. [Rocket](https://rocket.rs/) is way
  cleaner and perhaps viable now that the maintainer has returned from their hiatus.
- There are apparently no useful [OpenAPI](https://www.openapis.org/) documentation generator for Actix.
 

# The painful parts
- Rust is mighty picky about references and types. It is actually hard to figure out what to do in various places.
- Figuring out what type to use when mapping structs to a database schema can be tricky. The docs are not great in this
  regard.