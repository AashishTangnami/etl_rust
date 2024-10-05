use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::net:: {TcpListener, TcpStream};
use std::io::{ Read, Write};

// const DB_URL = !env("DATABASE_URL");

fn establish_connection() -> PgConnection {
    // Load environment variables from the .env file
    dotenv().ok();

    // Get the DATABASE_URL from environment variables
    let database_url : &str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection to the database
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    // Call the connection function
    let _connection = establish_connection();
    println!("Successfully connected to the database!");
}
