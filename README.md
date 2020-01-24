# Food-terest (temporary name)
Half-assed Pinterest-like (ideally) CRUD webservice intended to work as an recipe aggregator. Built with Actix.

## Getting started
* Install Rust
* Install SQLite
* Install the Diesel CLI with the sqlite feature enabled
* Copy .env.example to .env within this directory, and change environmental variables according to your system
* Setup your database by running 'diesel database setup'
* Build this project with 'cargo build'
* Run with 'cargo run'
* URL will be 'localhost:8080'